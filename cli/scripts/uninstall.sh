#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
cd "$script_dir/.."

install_prefix="$1"
case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo Removing binary
rm "$install_prefix/bin/openscq30_cli$bin_ext" || true
echo Removing bash completions
rm "$install_prefix/share/bash-completions/completions/openscq30_cli" || true
