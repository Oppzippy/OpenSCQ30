#!/usr/bin/env bash
set -e
set -o pipefail

# in format "commit_hash commit_message"
get_commits_since_tag() {
    git log "$1"..HEAD --oneline
}

# errors if there are no tags
get_latest_tag() {
    git describe --tags --abbrev=0
}

# $1: current version
# $2: place to increment (1 for x.0.0, 2 for 0.x.0, 3 for 0.0.x)
bump_version() {
    echo "$1" | \
        gawk \
            --assign increment_place="$2" \
            --source \
            'match($0, /([0-9]+)\.([0-9]+)\.([0-9]+)/, version) {
                version[increment_place] += 1;
                for (i = increment_place+1; i <= 3; i++) {
                    version[i] = 0;
                }

                print "v" \
                    version[1] "." \
                    version[2] "." \
                    version[3];
            }'
}

latest_tag=$(get_latest_tag)
commits=$(get_commits_since_tag "$latest_tag")
# for case insensitivity
commits_lower_case=${commits,,}

# allow for "type:" or "type(scope):"
if [[ ! -z $(echo "$commits_lower_case" | grep -E "^\w+ breaking change(\([^\)]*\))?: ") ]]; then
    bump_version "$latest_tag" 1
elif [[ ! -z $(echo "$commits_lower_case" | grep -E "^\w+ feat(\([^\)]*\))?: ") ]]; then
    bump_version "$latest_tag" 2
elif [[ ! -z "$commits" ]]; then
    bump_version "$latest_tag" 3
else
    echo "$latest_tag"
fi
