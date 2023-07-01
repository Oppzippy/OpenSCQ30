#!/usr/bin/env bash
set -e
set -o pipefail

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
    if [[ -z $(grep "<release version=\"$version\"" "$metainfo_file" ) ]]; then
        date=$(date --utc +"%Y-%m-%d")
        changelog=$(get_changelog_for_version "$changelog_file" "$version")
        escaped_changelog=$(printf '%s' "$changelog" | sed 's/[&/\]/\\&/g; s/$/\\/')
        sed --in-place --regexp-extended --null-data "s/<releases>/<releases>\n        <release version=\"$version\" date=\"$date\">\n            <p>$escaped_changelog<\/p>\n        <\/release>/" "$metainfo_file"
    fi
}

get_changelog_for_version() {
    changelog_file="$1"
    version="$2"
    awk --assign version="## v$version" '$0 ~ version {flag=1;next};/## v/{flag=0}flag' $changelog_file
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
set_version_in_cargo_toml android/Cargo.toml "$1"
set_version_in_cargo_toml cli/Cargo.toml "$1"
set_version_in_cargo_toml gui/Cargo.toml "$1"
set_version_in_cargo_toml lib/Cargo.toml "$1"
set_version_in_cargo_toml web/wasm/Cargo.toml "$1"
set_version_in_build_gradle android/app/build.gradle.kts "$1"
set_version_in_package_json web/package.json "$1"
set_version_in_appimage_builder packaging/appimage/AppImageBuilder.yml "$1"
set_version_in_iss packaging/windows/setup.iss "$1"
add_version_to_appstream_metainfo gui/resources/com.oppzippy.OpenSCQ30.metainfo.xml "$1" CHANGELOG.md
