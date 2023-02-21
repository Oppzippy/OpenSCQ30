#!/bin/bash
./flatpak-builder-tools/cargo/flatpak-cargo-generator.py ../Cargo.lock -o generated-sources.json
flatpak-builder --install repo com.oppzippy.OpenSCQ30.yml --user
