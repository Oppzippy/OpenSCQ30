#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."
input_binary="$project_root/build-output/openscq30-gui"
app_dir="$project_root/build-output/OpenSCQ30.app"

rm -rf "$app_dir"
mkdir -p "$app_dir/Contents/MacOS"

cp "$script_dir/Info.plist" "$app_dir/Contents/Info.plist"
cp "$input_binary" "$app_dir/Contents/MacOS/openscq30-gui"
chmod +x "$app_dir/Contents/MacOS/openscq30-gui"

# Ad-hoc signature so Launch Services identifies the bundle correctly and Gatekeeper allows
# running a locally built app. Not a substitute for real notarization: downloaded (quarantined)
# builds still need right-click -> Open.
codesign --force --sign - "$app_dir"
