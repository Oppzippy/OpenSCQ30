#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."
gui_dir="$project_root/gui"

install_prefix="$1"

case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo "Installing to $1"
install -Dm755 "$project_root/target/release/openscq30_gui$bin_ext" -t "$install_prefix/bin/"
install -Dm644 "$gui_dir/resources/com.oppzippy.OpenSCQ30.desktop" -t "$install_prefix/share/applications"
install -Dm644 "$gui_dir/resources/com.oppzippy.OpenSCQ30.metainfo.xml" -t "$install_prefix/share/metainfo"
install -Dm644 "$gui_dir/resources/com.oppzippy.OpenSCQ30.svg" -t "$install_prefix/share/icons/hicolor/scalable/apps"
cp -r "$gui_dir"/share/. "$install_prefix/share"
