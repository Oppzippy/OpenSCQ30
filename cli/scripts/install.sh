#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_root="$script_dir/../.."

install_path="$1"

echo Installing binary
mkdir -p "$install_path/bin"
install -Dm755 "$project_root/target/release/openscq30" -t "$install_path/bin/"

if [[ -z "${OPENSCQ30_SKIP_SHELL_COMPLETIONS:-}" ]]; then
    # bash
    if command -v bash > /dev/null; then
        echo Installing bash completions
        mkdir -p "$install_path/share/bash-completions/completions"
        "$install_path/bin/openscq30" completions --shell bash > "$install_path/share/bash-completions/completions/openscq30"
    else
        echo Skipping bash completions
    fi

    # fish
    if command -v fish > /dev/null; then
        echo Installing fish completions
        mkdir -p "$install_path/share/fish/completions"
        "$install_path/bin/openscq30" completions --shell fish > "$install_path/share/fish/completions/openscq30.fish"
    else
        echo Skipping fish completions
    fi

    # zsh
    if command -v zsh > /dev/null; then
        echo Installing zsh completions
        mkdir -p "$install_path/share/zsh/site-functions"
        "$install_path/bin/openscq30" completions --shell zsh > "$install_path/share/zsh/site-functions/_openscq30"
    else
        echo Skipping zsh completions
    fi
fi
