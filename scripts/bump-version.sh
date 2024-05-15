#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"

next_version=$(bash "$script_dir/get-next-version.sh")
echo "Setting version to $next_version"
bash "$script_dir/set-version.sh" "$next_version"
