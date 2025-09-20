#!/usr/bin/env bash
set -euo pipefail

install_path="$1"
case "$OSTYPE" in
    msys | cygwin) bin_ext=".exe" ;;
    *) bin_ext="" ;;
esac

echo "Uninstalling from $install_path"
echo Removing binary
rm "$install_path/bin/openscq30-gui" || true
echo Removing desktop file
rm "$install_path/share/applications/com.oppzippy.OpenSCQ30.desktop" || true
echo Removing appstream metainfo
rm "$install_path/share/metainfo/com.oppzippy.OpenSCQ30.metainfo.xml" || true
echo Removing icon
rm "$install_path/share/icons/hicolor/scalable/apps/com.oppzippy.OpenSCQ30.svg" || true
