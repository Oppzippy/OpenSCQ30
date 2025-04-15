#!/usr/bin/env bash
set -euo pipefail

install_path="$1"
case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo Removing binary
rm "$install_path/bin/openscq30$bin_ext" || true
echo Removing bash completions
rm "$install_path/share/bash-completions/completions/openscq30" || true
