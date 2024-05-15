#!/usr/bin/env bash
set -euo pipefail

set_version_in_changelog() {
    sed --in-place --regexp-extended "s/## Unreleased/## v$2/" "$1"
}

set_version_in_cargo_toml() {
    # --null-data treats the entire file as one line. this makes it easy to only replace the first occurrance of
    # "version =" to avoid catching any dependency versions
    sed --in-place --regexp-extended --null-data "s/(\nversion *= *\")([0-9\.]+)(\"\n)/\1$2\3/" "$1"
}

set_version_in_build_gradle() {
    sed --in-place --regexp-extended --null-data "s/(\n *versionName *= *\")([0-9\.]+)(\"\n)/\1$2\3/" "$1"
    next_version_code=$(($(git tag | wc -l) + 1))
    sed --in-place --regexp-extended --null-data "s/(\n *versionCode *= *)([0-9]+)(\n)/\1$next_version_code\3/" "$1"
}

set_version_in_package_json() {
    # --null-data treats the entire file as one line. this makes it easy to only replace the first occurrance of
    # "version =" to avoid catching any dependency versions
    sed --in-place --regexp-extended --null-data "s/(\n *\"version\": *\")([0-9\.]+)(\",\n)/\1$2\3/" "$1"
}

set_version_in_appimage_builder() {
    sed --in-place --regexp-extended --null-data "s/(\n *version: *)([0-9\.]+)( *\n)/\1$2\3/" "$1"
}

set_version_in_iss() {
    sed --in-place --regexp-extended --null-data "s/(\n#define AppVersion \")([0-9\.]+)(\"\n)/\1$2\3/" "$1"
}

add_version_to_appstream_metainfo() {
    metainfo_file="$1"
    version="$2"
    changelog_file="$3"
    # If the release is already listed, don't add it again
    if ! grep --quiet "<release version=\"$version\"" "$metainfo_file"; then
        date=$(date --utc +"%Y-%m-%d")
        changelog_markdown=$(get_changelog_for_version "$changelog_file" "$version")
        changelog_html=$(printf '%s' "$changelog_markdown" | gawk '
            # Skip empty lines
            ! /\W/ {
                next
            }
            # GUI, Android, etc.
            /^### / {
                category = gensub(/^### (.*)/, "\\1", "g", $0)
                next
            }
            # Features, Fixes, etc
            /^#### / {
                subcategory = gensub(/^#### (.*)/, "\\1", "g", $0)
                next
            }
            {
                if (category == "General" || category == "GUI") {
                    # Trim hyphen for markdown unordered lists
                    line_text = gensub(/^- +(.*)/, "\\1", "g", $0)
                    if (line_text) {
                        sections[subcategory][length(sections[subcategory])+1] = line_text
                    }
                }
            }
            END {
                for (s in sections) {
                    print "<p>" s "</p>"
                    print "<ul>"
                    for (i = 1; i <= length(sections[s]); i++) {
                        print "    <li>" sections[s][i] "</li>"
                    }
                    print "</ul>"
                }
            }
        ' | awk '{
            # Add indentation to match the surrounding xml tag
            print "                " $0
        }')

        escaped_changelog=$(printf '%s' "$changelog_html" | sed 's/[&/\]/\\&/g; s/$/\\/')

        sed --in-place --regexp-extended --null-data "s/<releases>/<releases>\n        <release version=\"$version\" date=\"$date\">\n            <description>\n$escaped_changelog \n            <\/description>\n        <\/release>/" "$metainfo_file"
    fi
}

get_changelog_for_version() {
    changelog_file="$1"
    version="$2"
    awk --assign version="## v$version" '$0 ~ version {flag=1;next};/## v/{flag=0}flag' "$changelog_file"
}

if [ -z "$1" ]; then
    cat <<EOF
Usage: set-version.sh 1.0.0
EOF
    exit 1
fi

if [[ ! "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo Version number format must be x.y.z
    exit 2
fi

set_version_in_changelog CHANGELOG.md "$1"
set_version_in_cargo_toml Cargo.toml "$1"
set_version_in_build_gradle android/app/build.gradle.kts "$1"
set_version_in_package_json web/package.json "$1"
set_version_in_appimage_builder packaging/appimage/AppImageBuilder.yml "$1"
set_version_in_iss packaging/windows/setup.iss "$1"
add_version_to_appstream_metainfo gui/resources/com.oppzippy.OpenSCQ30.metainfo.xml "$1" CHANGELOG.md
