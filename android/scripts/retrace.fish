#!/usr/bin/env fish

if ! command --query retrace
    echo "retrace must be in PATH. See: https://developer.android.com/tools/retrace" >&2
    exit 1
end

argparse "h/help" "p/profile=" -- $argv
or return

if set --local --query _flag_h
    echo "Usage: ./retrace.fish [-h | --help] [-p | --profile=BUILD_PROFILE] [stacktrace-file]" >&2
    exit
end

set --local profile release-arm64-v8a

set --local --query _flag_profile[1]
and set profile $_flag_profile[1]

set --local android_project_root $(dirname $(status --current-filename))/../

set --local mapping_path $android_project_root/app/build/outputs/mapping/$profile/mapping.txt

retrace $mapping_path $argv[1]
