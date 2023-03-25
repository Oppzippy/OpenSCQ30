#!/usr/bin/env bash
set -e
set -o pipefail
PROJECT_NAME="com.oppzippy.OpenSCQ30"

return_dir=$(pwd)
script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- $script_path)"
cd "$script_dir/../gui"

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
    if [[ ! -d "po/$1" ]]; then
        mkdir "po/$1"
    fi
    if [[ ! -f "po/$1/$PROJECT_NAME.po" ]]; then
        msginit --input "po/$PROJECT_NAME.pot" --locale "$1" --output "po/$1/$PROJECT_NAME.po"
    else
        msgmerge --update "po/$1/$PROJECT_NAME.po" "po/$PROJECT_NAME.pot"
    fi
    mkdir -p "locales/$1/LC_MESSAGES"
    msgfmt --output-file "locales/$1/LC_MESSAGES/$PROJECT_NAME.mo" "po/$1/$PROJECT_NAME.po"
}

while read line; do
    if [[ ! -z "$line" ]]; then
        update_locale "$line"
    fi
done <po/LINGUAS

cd "$return_dir"
