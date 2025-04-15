#!/usr/bin/env bash
set -euo pipefail

install_path="$1"
case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo Removing binary
rm "$install_path/bin/openscq30_gui$bin_ext" || true
