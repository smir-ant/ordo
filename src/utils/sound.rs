/// OGG Vorbis decoder + audio playback via Makepad AudioBuffer.
/// Decoded samples stored as mono f32; resampled on-the-fly in audio callback.
use std::sync::{Arc, Mutex};
use makepad_widgets::makepad_platform::audio::*;

pub struct DecodedSound {
    pub samples: Vec<f32>, // mono, normalized –1..1
    pub sample_rate: u32,
}

struct Playback {
    sound_idx: usize,
    position: f64, // fractional index for linear interpolation
}

pub struct SoundPlayerInner {
    sounds: Vec<DecodedSound>,
    active: Option<Playback>, // None = silence; Some = playing
}

#[derive(Clone)]
pub struct SoundPlayer {
    inner: Arc<Mutex<SoundPlayerInner>>,
}

impl DecodedSound {
    /// Decode OGG Vorbis bytes → mono f32 via lewton.
    /// Stereo/multi-channel files are mixed down to mono.
    pub fn from_ogg(data: &[u8]) -> Self {
        use lewton::inside_ogg::OggStreamReader;

        let mut reader = OggStreamReader::new(std::io::Cursor::new(data))
            .expect("OGG open failed");

        let sample_rate = reader.ident_hdr.audio_sample_rate;
        let channels = reader.ident_hdr.audio_channels as usize;
        let mut samples = Vec::new();

        // lewton yields interleaved i16 packets
        while let Some(packet) = reader.read_dec_packet_itl().expect("OGG decode failed") {
            if channels == 1 {
                samples.extend(packet.iter().map(|&s| s as f32 / 32768.0));
            } else {
                // mixdown to mono: average all channels per frame
                for frame in packet.chunks(channels) {
                    let sum: f32 = frame.iter().map(|&s| s as f32 / 32768.0).sum();
                    samples.push(sum / channels as f32);
                }
            }
        }

        DecodedSound { samples, sample_rate }
    }
}

impl SoundPlayer {
    pub fn new() -> Self {
        SoundPlayer {
            inner: Arc::new(Mutex::new(SoundPlayerInner {
                sounds: Vec::new(),
                active: None,
            })),
        }
    }

    /// Arc clone for passing into audio_output closure (must be Send + 'static)
    pub fn inner_arc(&self) -> Arc<Mutex<SoundPlayerInner>> {
        Arc::clone(&self.inner)
    }

    /// Store decoded sound, return its index
    pub fn load(&self, sound: DecodedSound) -> usize {
        let mut inner = self.inner.lock().unwrap();
        let idx = inner.sounds.len();
        inner.sounds.push(sound);
        idx
    }

    /// Start (or restart) playback from the beginning
    pub fn play(&self, idx: usize) {
        self.inner.lock().unwrap().active = Some(Playback {
            sound_idx: idx,
            position: 0.0,
        });
    }

    /// Audio thread callback — fills buffer with resampled mono samples.
    /// Uses try_lock to never block the audio thread (silent on contention).
    pub fn audio_callback(inner: &Arc<Mutex<SoundPlayerInner>>, info: AudioInfo, buffer: &mut AudioBuffer) {
        let mut guard = match inner.try_lock() {
            Ok(g) => g,
            Err(_) => {
                buffer.zero(); // contention → silent buffer (~5ms)
                return;
            }
        };

        let (sound_idx, mut position) = match guard.active {
            Some(ref p) => (p.sound_idx, p.position),
            None => {
                buffer.zero();
                return;
            }
        };

        let sound = &guard.sounds[sound_idx];
        let rate_ratio = sound.sample_rate as f64 / info.sample_rate; // src/dst sample rate
        let src_len = sound.samples.len();
        let channels = buffer.channel_count();
        let frames = buffer.frame_count();

        for frame in 0..frames {
            let idx = position as usize;

            // Linear interpolation between adjacent samples
            let sample = if idx >= src_len {
                0.0
            } else {
                let s0 = sound.samples[idx];
                let s1 = if idx + 1 < src_len { sound.samples[idx + 1] } else { 0.0 };
                let frac = (position - idx as f64) as f32;
                s0 + (s1 - s0) * frac
            };

            // Write mono sample to all output channels
            for channel in 0..channels {
                buffer.channel_mut(channel)[frame] = sample;
            }

            position += rate_ratio;
        }

        // Stop playback when sound is finished
        if position as usize >= src_len {
            guard.active = None;
        } else {
            guard.active.as_mut().unwrap().position = position;
        }
    }
}
