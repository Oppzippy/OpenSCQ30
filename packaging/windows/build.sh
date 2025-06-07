#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."
input_exe="$project_root/target/release/openscq30-gui.exe"

# Fetch dependencies
if [[ ! -d "$script_dir/dependencies/cosmic-icons" ]]; then
    git clone --depth 1 https://github.com/pop-os/cosmic-icons.git "$script_dir/dependencies/cosmic-icons"
fi

# Copy files into installation directory
if [[ -d "$script_dir/root" ]]; then
    rm -rf "$script_dir/root"
fi
mkdir --parents "$script_dir/root/bin"
mkdir --parents "$script_dir/root/share/icons/Cosmic/scalable"

cp "$input_exe" "$script_dir/root/bin/"
cp --archive \
    "$script_dir/dependencies/cosmic-icons/freedesktop/scalable/." \
    "$script_dir/root/share/icons/Cosmic/scalable/"

ISCC "$script_dir/setup.iss"
