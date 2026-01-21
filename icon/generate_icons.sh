#!/bin/bash
set -e

# Working directory should be the 'icon' folder
SOURCE="AppIcon.png"
MAC_DEST="AppIcon.icns"
ANDROID_RES="../makepad/tools/cargo_makepad/src/android/res"

# Colors (Inverted: Black Icon on White Background)
BG_COLOR="#FFFFFF"
FILL_COLOR="#000000"

# Path Data (Full)
PATH_DATA="M312.463989,806.793091   C282.305725,804.213196 258.116241,790.557312 237.829453,769.455933   C233.675217,765.134827 230.107727,764.191345 224.380066,766.081604   C212.218430,770.095337 199.712860,773.087769 187.028839,774.989441   C183.317291,775.545959 181.601547,777.141724 180.212051,780.379700   C172.659225,797.980286 164.929535,815.505249 157.214783,833.036011   C155.811172,836.225525 154.478317,839.491394 151.520279,841.700012   C146.101669,845.745972 139.685303,845.890625 134.370438,842.028442   C129.265549,838.318665 127.066963,830.877625 129.746765,824.493958   C135.354523,811.135315 141.262024,797.902710 147.009323,784.602295   C148.114777,782.044067 149.716171,779.641174 149.864319,776.353638   C146.849411,775.479553 143.822586,774.691956 140.853683,773.726196   C125.171341,768.624573 114.278679,758.562927 111.984283,741.910461   C109.535194,724.135315 117.902649,710.893372 132.975586,702.029480   C147.866684,693.272522 164.157837,691.813232 180.830444,695.048889   C185.344604,695.924927 187.049988,694.552551 188.606216,690.730042   C199.764450,663.322876 207.199768,634.868591 212.670044,605.864197   C218.869156,572.995300 222.782837,539.827698 225.894943,506.532257   C229.140991,471.803955 229.829590,437.025574 228.571869,402.225372   C227.341431,368.179443 221.055847,334.818726 213.265396,301.722626   C207.138535,275.693878 203.272888,249.384689 204.748962,222.556686   C206.662292,187.781555 217.283554,156.609406 243.384277,132.158432   C265.098877,111.816353 291.329437,100.432083 320.165100,95.012169   C357.382263,88.016853 394.120422,90.320488 429.884888,103.262100   C479.184906,121.101639 515.157166,154.228058 539.662903,200.163910   C555.670105,230.169235 564.972229,262.294525 569.435486,295.947113   C571.913330,314.629974 573.619507,333.399994 572.946472,352.184082   C569.900085,437.206146 548.056213,516.453796 498.927673,586.961853   C469.584106,629.075073 432.351013,663.014160 389.619659,691.154663   C352.138062,715.838074 311.883270,734.902466 269.934845,750.572815   C268.534454,751.095947 267.156281,751.681152 265.784973,752.276978   C265.504822,752.398743 265.302124,752.698730 264.774109,753.181213   C268.263062,757.935791 273.289124,760.930664 278.003571,764.099792   C294.650787,775.289978 312.850067,779.504883 332.671387,775.136292   C338.296814,773.896484 343.771729,773.036560 349.236694,775.787537   C354.142487,778.257080 356.794830,782.183960 357.306000,787.562805   C357.854156,793.330566 355.971985,798.019714 350.966034,801.280457   C346.794617,803.997559 342.108917,804.938477 337.288177,805.620361   C329.196045,806.765137 321.108368,807.746033 312.463989,806.793091  M544.776123,354.470490   C545.620605,325.881287 543.432861,297.578705 536.713684,269.710876   C528.515625,235.709091 515.223511,203.919037 491.608124,177.773376   C442.100433,122.961227 380.474121,107.565460 309.985291,125.443848   C284.977478,131.786682 263.729340,145.064743 249.093933,167.160812   C236.443497,186.260071 231.927948,207.788986 231.379822,230.259079   C230.737457,256.592865 235.977997,282.204102 241.979996,307.656097   C249.096619,337.834747 254.023712,368.351868 256.026215,399.289215   C257.663269,424.580353 257.259399,449.907288 255.946503,475.205353   C255.196152,489.663818 254.274902,504.123016 252.979843,518.541382   C250.882034,541.897034 247.954849,565.157715 244.396790,588.344910   C238.372726,627.602844 229.937347,666.218140 214.530350,703.009338   C213.199173,706.188049 213.885757,707.728149 216.614136,709.418152   C224.154800,714.088928 230.966461,719.762512 237.001480,726.220764   C240.138336,729.577637 242.988205,729.920837 247.127029,728.427979   C283.244110,715.400330 318.271240,700.013672 351.462738,680.628479   C392.354462,656.746216 429.419006,628.102600 459.464355,591.225220   C515.341431,522.642212 540.602417,442.829651 544.776123,354.470490  M167.014542,738.541626   C168.667343,733.576477 172.149445,729.334595 173.172943,723.660828   C164.459915,722.679504 156.291656,722.794922 148.636414,726.862915   C144.499237,729.061340 140.967560,732.010559 141.293427,737.192627   C141.630615,742.554199 145.879623,744.514160 150.268890,745.987488   C160.579224,749.448425 162.121094,748.790894 167.014542,738.541626  M201.330231,735.070801   C198.228516,736.912720 197.712067,739.998657 196.987137,743.541626   C201.091812,742.934204 204.782425,742.746582 208.044128,740.400696   C206.553970,737.725647 204.028610,736.740601 201.330231,735.070801  z"

if [ ! -f "$SOURCE" ]; then
    echo "Source icon not found at $SOURCE"
    exit 1
fi

# =========================================================================
# macOS Icons
# =========================================================================
echo "Generating macOS icons..."
ICONSET="AppIcon.iconset"
mkdir -p "$ICONSET"
sips -z 16 16     "$SOURCE" --out "$ICONSET/icon_16x16.png" > /dev/null
sips -z 32 32     "$SOURCE" --out "$ICONSET/icon_16x16@2x.png" > /dev/null
sips -z 32 32     "$SOURCE" --out "$ICONSET/icon_32x32.png" > /dev/null
sips -z 64 64     "$SOURCE" --out "$ICONSET/icon_32x32@2x.png" > /dev/null
sips -z 128 128   "$SOURCE" --out "$ICONSET/icon_128x128.png" > /dev/null
sips -z 256 256   "$SOURCE" --out "$ICONSET/icon_128x128@2x.png" > /dev/null
sips -z 256 256   "$SOURCE" --out "$ICONSET/icon_256x256.png" > /dev/null
sips -z 512 512   "$SOURCE" --out "$ICONSET/icon_256x256@2x.png" > /dev/null
sips -z 512 512   "$SOURCE" --out "$ICONSET/icon_512x512.png" > /dev/null
sips -z 1024 1024 "$SOURCE" --out "$ICONSET/icon_512x512@2x.png" > /dev/null 
iconutil -c icns "$ICONSET"
rm -rf "$ICONSET"
echo "MacOS icon created at $(pwd)/$MAC_DEST"

# =========================================================================
# Android Icons (Adaptive + Legacy)
# =========================================================================
echo "Generating Android icons (Optimized Vector + Legacy)..."

# 1. Create Directories/Values
mkdir -p "$ANDROID_RES/values"
mkdir -p "$ANDROID_RES/drawable"
mkdir -p "$ANDROID_RES/mipmap-anydpi-v26"
mkdir -p "$ANDROID_RES/mipmap-mdpi"
mkdir -p "$ANDROID_RES/mipmap-hdpi"
mkdir -p "$ANDROID_RES/mipmap-xhdpi"
mkdir -p "$ANDROID_RES/mipmap-xxhdpi"
mkdir -p "$ANDROID_RES/mipmap-xxxhdpi"

# 2. Define Background Color (ic_launcher_background.xml)
cat > "$ANDROID_RES/values/ic_launcher_background.xml" <<EOF
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">$BG_COLOR</color>
</resources>
EOF

# 3. Define Vector Foreground (ic_launcher_foreground.xml)
# Tweaked Padding
# Viewport 1800x1800 (Scale ~42% of icon size)
# Center 900,900
# Logo Center (342, 468) -> Shift (+558, +432)
cat > "$ANDROID_RES/drawable/ic_launcher_foreground.xml" <<EOF
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="108dp"
    android:height="108dp"
    android:viewportWidth="1800"
    android:viewportHeight="1800">
    <group
        android:translateX="558"
        android:translateY="432">
        <path
            android:fillColor="$FILL_COLOR"
            android:pathData="$PATH_DATA" />
    </group>
</vector>
EOF

# 4. Define Adaptive Icon (ic_launcher.xml)
cat > "$ANDROID_RES/mipmap-anydpi-v26/ic_launcher.xml" <<EOF
<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
EOF

# 5. Generate Legacy Images (ic_launcher.png)
rm -f "$ANDROID_RES"/mipmap-*/ic_launcher_foreground.png

generate_legacy() {
    local legacy_size=$1
    local dir=$2
    sips -z $legacy_size $legacy_size "$SOURCE" --out "$ANDROID_RES/$dir/ic_launcher.png" > /dev/null
}

generate_legacy 48  "mipmap-mdpi"
generate_legacy 72  "mipmap-hdpi"
generate_legacy 96  "mipmap-xhdpi"
generate_legacy 144 "mipmap-xxhdpi"
generate_legacy 192 "mipmap-xxxhdpi"

# =========================================================================
# Optimize PNGs for minimal APK size
# =========================================================================
# pngquant uses lossy compression to significantly reduce PNG file size
# Quality 65-80 provides good balance between size and visual quality
# This step reduces total icon size from ~21KB to ~5KB (~75% reduction)
# Install: brew install pngquant
echo "Optimizing PNG icons with pngquant..."
if command -v pngquant &> /dev/null; then
    for dir in mipmap-mdpi mipmap-hdpi mipmap-xhdpi mipmap-xxhdpi mipmap-xxxhdpi; do
        pngquant --quality=65-80 --force --output "$ANDROID_RES/$dir/ic_launcher.png" "$ANDROID_RES/$dir/ic_launcher.png"
    done
    echo "PNG optimization complete!"
else
    echo "Warning: pngquant not found. Skipping optimization. Install with: brew install pngquant"
fi

echo "Android icons updated in $ANDROID_RES"
