use super::loader::{FontDefinition, FontFamilyDefinition, Loader};

pub const OPTIMIZED_FONT: &[u8] = include_bytes!("../../../../src/embeds/Inter_opt.ttf");

pub fn define(loader: &mut Loader) {
    // Define the single shared font definition (Inter)
    let inter_def = FontDefinition {
        data: OPTIMIZED_FONT.to_vec().into(),
        index: 0,
        ascender_fudge_in_ems: 0.0,
        descender_fudge_in_ems: 0.0,
    };
    
    // Define the "Sans" font and family
    loader.define_font("Sans".into(), inter_def.clone());
    loader.define_font_family(
        "Sans".into(),
        FontFamilyDefinition {
            font_ids: ["Sans".into()].into(),
        },
    );

    // Define the "UbuntuMono" font and family -> Alias to Inter
    loader.define_font("UbuntuMono".into(), inter_def.clone());
    loader.define_font_family(
        "UbuntuMono".into(),
        FontFamilyDefinition {
            font_ids: ["UbuntuMono".into()].into(),
        },
    );
    
    // Define the "Monospace" alias -> Alias to Inter
    loader.define_font("Monospace".into(), inter_def.clone());
    loader.define_font_family(
        "Monospace".into(),
        FontFamilyDefinition {
            font_ids: ["Monospace".into()].into(),
        },
    );

    // Define the Empty "" font and family (Default fallback)
    loader.define_font("".into(), inter_def.clone());
    loader.define_font_family(
        "".into(),
        FontFamilyDefinition {
            font_ids: ["".into()].into(),
        },
    );
}
