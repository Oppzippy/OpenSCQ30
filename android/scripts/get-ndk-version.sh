#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
build_script_path="$script_dir/../app/build.gradle.kts"

awk -F '"' '/ndkVersion = / { print $2 }' "$build_script_path"

