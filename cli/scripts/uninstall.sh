#!/usr/bin/env bash
set -euo pipefail

install_path="$1"

echo Removing binary
rm "$install_path/bin/openscq30" || true
echo Removing bash completions
rm "$install_path/share/bash-completions/completions/openscq30" || true
echo Removing fish completions
rm "$install_path/share/fish/completions/openscq30.fish" || true
echo Removing zsh completions
rm "$install_path/share/zsh/site-functions/_openscq30" || true
