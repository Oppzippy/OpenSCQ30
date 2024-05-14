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

echo Installing binary
install -Dm755 "../target/release/openscq30_cli$bin_ext" -t "$install_prefix/bin/"

if [[ -z "$OPENSCQ30_SKIP_SHELL_COMPLETIONS" ]]; then
    if [[ -d "$install_prefix/share/bash-completions/completions" ]]; then
        echo Installing bash completions
        "../target/release/openscq30_cli$bin_ext" completions bash > "$install_prefix/share/bash-completions/completions/openscq30_cli"
    else
        echo Skipping bash completions
    fi
fi
