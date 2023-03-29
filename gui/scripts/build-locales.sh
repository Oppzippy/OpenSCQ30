#!/usr/bin/env bash
set -e
set -o pipefail
PROJECT_NAME="com.oppzippy.OpenSCQ30"

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- $script_path)"
gui_dir="$script_dir/.."
out_dir=$1

if [[ -z $out_dir ]]; then
    out_dir="$gui_dir"
fi


existing_locales=$(find po -mindepth 1 -maxdepth 1 -type d | sed 's/^po\///')

while read locale; do
    echo "Building $locale"
    mkdir -p "$out_dir/share/locale/$locale/LC_MESSAGES"
    msgfmt --output-file "$out_dir/share/locale/$locale/LC_MESSAGES/$PROJECT_NAME.mo" "$gui_dir/po/$locale/$PROJECT_NAME.po"
done <<< "$existing_locales"
