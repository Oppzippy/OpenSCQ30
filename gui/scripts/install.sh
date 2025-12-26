#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."

install_path="$1"

echo Installing binary
install -Dm755 "$project_root/build-output/openscq30-gui" -t "$install_path/bin/"
echo Installing desktop file
install -Dm644 "$project_root/gui/resources/com.oppzippy.OpenSCQ30.desktop" -t "$install_path/share/applications"
echo Installing appstream metadata
install -Dm644 "$project_root/gui/resources/com.oppzippy.OpenSCQ30.metainfo.xml" -t "$install_path/share/metainfo"
echo Installing application icon
install -Dm644 "$project_root/gui/resources/com.oppzippy.OpenSCQ30.svg" -t "$install_path/share/icons/hicolor/scalable/apps"

