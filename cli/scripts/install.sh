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
install -Dm755 "$project_root/target/release/openscq30$bin_ext" -t "$install_path/bin/"

if [[ -z "${OPENSCQ30_SKIP_SHELL_COMPLETIONS:-}" ]]; then
    if [[ -d "$install_path/share/bash-completions/completions" ]]; then
        echo Installing bash completions
        "$project_root/target/release/openscq30$bin_ext" completions bash > "$install_path/share/bash-completions/completions/openscq30"
    else
        echo Skipping bash completions
    fi
fi
