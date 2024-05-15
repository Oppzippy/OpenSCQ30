#!/usr/bin/env bash
set -euo pipefail
PROJECT_NAME="com.oppzippy.OpenSCQ30"

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
gui_dir="$script_dir/.."

existing_locales=$(find "$gui_dir/po" -mindepth 1 -maxdepth 1 -type d -printf '%f\n')

# Default locale is en
echo "Building en"
mkdir -p "$gui_dir/share/locale/en/LC_MESSAGES"
msgfmt --output-file "$gui_dir/share/locale/en/LC_MESSAGES/$PROJECT_NAME.mo" "$gui_dir/po/$PROJECT_NAME.pot"

while read -r locale; do
    echo "Building $locale"
    mkdir -p "$gui_dir/share/locale/$locale/LC_MESSAGES"
    msgfmt --output-file "$gui_dir/share/locale/$locale/LC_MESSAGES/$PROJECT_NAME.mo" "$gui_dir/po/$locale/$PROJECT_NAME.po"
done <<< "$existing_locales"
