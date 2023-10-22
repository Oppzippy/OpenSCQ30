#!/usr/bin/env bash
set -e
set -o pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- $script_path)"
project_root="$script_dir/../.."
input_exe="$project_root/target/release/openscq30_gui.exe"

package_type="ucrt64"
output_dir="$script_dir/root"

# Install OpenSCQ30
export CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS=gui
export INSTALL_PREFIX="$output_dir"
cargo make --profile release --cwd "$project_root" build
cargo make --profile release --cwd "$project_root" install

# Determine what dlls are required, and which packages those dlls belong to
list=$(ldd "$input_exe" | grep "/$package_type" | sed "s/.dll.*/.dll/")
for dll in $list;
do
    package=$(pacman -Qo "$dll" | awk '{ print $5 }')
    dependencies="$dependencies\n$package"
done
dependencies=$(printf "$dependencies" | sort | uniq)
echo "packages: $dependencies"

# Find all dll files belonging to packages that own required dll files
files=$(pacman -Ql $dependencies | sed -E "s/^[^ ]+ //" | grep -E "^\/$package_type\/bin/.*\.dll" | sort | uniq)
echo "files: $files"

# Install the files, stripping the /ucrt64 prefix
for file in $files; do
    if [[ -f "$file" ]]; then
        file_without_prefix=$(echo $file | sed -E "s/^\/$package_type//")
        output_file="$output_dir/$file_without_prefix"
        output_file_dir=$(dirname "$output_file")
        mkdir -p "$output_file_dir"
        cp "$file" "$output_file"
    fi
done

# Install other dependencies
mkdir -p "$output_dir/share/glib-2.0"
cp -r "/$package_type/bin/gdbus.exe" "$output_dir/bin/"
cp -r "/$package_type/share/locale" "$output_dir/share/"
cp -r "/$package_type/share/glib-2.0/schemas" "$output_dir/share/glib-2.0/"
