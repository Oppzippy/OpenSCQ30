#!/usr/bin/env bash
set -e
set -o pipefail
PROJECT_NAME="com.oppzippy.OpenSCQ30"

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- $script_path)"
cd "$script_dir/../gui"

build_locale() {
    mkdir -p "locale/$1/LC_MESSAGES"
    msgfmt --output-file "locale/$1/LC_MESSAGES/$PROJECT_NAME.mo" "po/$1/$PROJECT_NAME.po"
}

existing_locales=$(find po -mindepth 1 -maxdepth 1 -type d | sed 's/^po\///')
while read locale; do
    echo "Building $locale"
    build_locale "$locale"
done <<< "$existing_locales"
