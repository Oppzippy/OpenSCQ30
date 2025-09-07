#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."
input_exe="$project_root/target/release/openscq30-gui.exe"

cp "$input_exe" "$script_dir/root/"

ISCC "$script_dir/setup.iss"
