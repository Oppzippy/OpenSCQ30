#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
ftl_path="$(dirname -- "$script_path")/../lib/i18n/en/openscq30-lib.ftl"

search_term="$1"

if [[ "$search_term" =~ ^a[0-9]+$ ]]; then
    sed -E -n "s/^soundcore-$search_term *= *(.*)$/\1/p" "$ftl_path"
else
    sed -E -n "s/^soundcore-(a[0-9]+) *= *(.*$search_term.*)$/\1/ip" "$ftl_path"
fi
