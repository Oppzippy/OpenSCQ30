#!/usr/bin/env bash
set -euo pipefail

script_path="$(readlink -f -- "$0")"
script_dir="$(dirname -- "$script_path")"
cd "$script_dir/.."

install_prefix="$1"

case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo "Uninstalling from $install_prefix"
echo Removing binary
rm "$install_prefix/bin/openscq30_gui$bin_ext" || true
echo Removing desktop file
rm "$install_prefix/share/applications/com.oppzippy.OpenSCQ30.desktop" || true
echo Removing appstream metainfo
rm "$install_prefix/share/metainfo/com.oppzippy.OpenSCQ30.metainfo.xml" || true
echo Removing icon
rm "$install_prefix/share/icons/hicolor/scalable/apps/com.oppzippy.OpenSCQ30.svg" || true
echo Removing locales
rm "$install_prefix"/share/locale/*/LC_MESSAGES/com.oppzippy.OpenSCQ30.mo || true
