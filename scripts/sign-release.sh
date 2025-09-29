#!/usr/bin/env bash
set -euo pipefail

unzip android.zip
unzip windows.zip
unzip linux.zip

read -s -p "$ANDROID_SIGNING_KEY password: " keystore_password

echo
echo Signing APKs
KEYSTORE_PASSWORD="$keystore_password" fd 'openscq30-android-.*-unsigned\.apk' --threads 1 -x apksigner sign --ks "$ANDROID_SIGNING_KEY" --ks-pass env:KEYSTORE_PASSWORD {}

echo Deleting .idsig files
rm openscq30-android-*.idsig

echo Renaming apk files
rename -- -unsigned.apk .apk openscq30-android-*.apk

echo Signing files
fd 'openscq30-.*' -x gpg --detach-sign {}

read -p "Delete zip files? [y/N] " delete_zip_files
if [[ "$delete_zip_files" == "y" ]]; then
    rm android.zip windows.zip linux.zip
fi

