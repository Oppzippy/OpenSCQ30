#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."
input_binary="$project_root/build-output/openscq30-gui"
app_dir="$project_root/build-output/OpenSCQ30.app"

icon_svg="$project_root/gui/resources/com.oppzippy.OpenSCQ30.svg"

rm -rf "$app_dir"
mkdir -p "$app_dir/Contents/MacOS" "$app_dir/Contents/Resources"

cp "$script_dir/Info.plist" "$app_dir/Contents/Info.plist"
cp "$input_binary" "$app_dir/Contents/MacOS/openscq30-gui"
chmod +x "$app_dir/Contents/MacOS/openscq30-gui"

# Generate AppIcon.icns from the shared SVG icon using tools that ship with macOS:
# qlmanage renders the SVG to a 1024x1024 PNG, sips downscales it, iconutil packs the iconset.
icon_tmp_dir="$(mktemp -d)"
trap 'rm -rf "$icon_tmp_dir"' EXIT
qlmanage -t -s 1024 -o "$icon_tmp_dir" "$icon_svg" >/dev/null
icon_png="$icon_tmp_dir/$(basename -- "$icon_svg").png"
if [[ ! -f "$icon_png" ]]; then
    echo "Failed to render $icon_svg to PNG" >&2
    exit 1
fi
iconset_dir="$icon_tmp_dir/AppIcon.iconset"
mkdir "$iconset_dir"
for size in 16 32 128 256 512; do
    sips -z "$size" "$size" "$icon_png" --out "$iconset_dir/icon_${size}x${size}.png" >/dev/null
    sips -z "$((size * 2))" "$((size * 2))" "$icon_png" --out "$iconset_dir/icon_${size}x${size}@2x.png" >/dev/null
done
iconutil --convert icns --output "$app_dir/Contents/Resources/AppIcon.icns" "$iconset_dir"

# Ad-hoc signature so Launch Services identifies the bundle correctly and Gatekeeper allows
# running a locally built app. Not a substitute for real notarization: downloaded (quarantined)
# builds still need right-click -> Open.
codesign --force --sign - "$app_dir"
