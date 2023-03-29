#!/usr/bin/env bash
set -e
set -o pipefail
PROJECT_NAME="com.oppzippy.OpenSCQ30"

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- $script_path)"
cd "$script_dir/.."

echo "Generating template"
if [[ -f "po/$PROJECT_NAME.pot" ]]; then
    xgettext --files-from po/POTFILES.in \
        --join-existing \
        --add-comments \
        --sort-output \
        --output "po/$PROJECT_NAME.pot"
else
    xgettext --files-from po/POTFILES.in \
        --add-comments \
        --sort-output \
        --output "po/$PROJECT_NAME.pot"
fi

update_locale() {
    echo "Updating $1"
    mkdir -p "po/$1"
    if [[ ! -f "po/$1/$PROJECT_NAME.po" ]]; then
        msginit --input "po/$PROJECT_NAME.pot" --locale "$1" --output "po/$1/$PROJECT_NAME.po"
    else
        msgmerge --update "po/$1/$PROJECT_NAME.po" "po/$PROJECT_NAME.pot"
    fi
}

while read line; do
    if [[ ! -z "$line" ]]; then
        update_locale "$line"
    fi
done <po/LINGUAS
