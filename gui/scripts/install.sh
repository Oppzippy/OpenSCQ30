#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."

install_path="$1"
case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo Installing binary
install -Dm755 "$project_root/target/release/openscq30_gui$bin_ext" -t "$install_path/bin/"
