#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
project_dir="$script_dir/.."
output_path="$project_dir/.build-tool-versions"

rustup_version=$(rustup --version 2>/dev/null | awk '{ print $2 }')
rust_version=$(rustup show | awk '$1 == "rustc" { print $2 }')
if [[ -f "$output_path" ]]; then
    java_version=$(awk '$1 == "jdk" { print $2 }' "$output_path")
else
    java_version="0"
fi
ndk_version=$("$project_dir/android/scripts/get-ndk-version.sh")
cargo_ndk_version=$(cargo ndk --version | awk '{ print $2 }')

cat <<EOF > "$output_path"
rustup $rustup_version
rust $rust_version
jdk $java_version
ndk $ndk_version
cargo-ndk $cargo_ndk_version
EOF
