mod android
mod cli
mod gui
mod i18n
mod i18n-macros
mod lib
mod lib-macros
mod lib-has

set unstable := true
fdfind := if which("fdfind") == "" { "fd" } else { "fdfind" }

default:
    @just --choose

build profile='dev':
    just gui/ build '{{ profile }}'
    just cli/ build '{{ profile }}'
    just android/ build '{{ profile }}'

test: lib::test cli::test gui::test android::test

test-cov: lib::test-cov cli::test-cov gui::test-cov android::test-cov

llvm-cov-clean:
    cargo llvm-cov clean --workspace

test-cov-report format='lcov':
    #!/usr/bin/env bash
    set -euo pipefail

    case '{{format}}' in
        lcov)
            format_args="--lcov --output-path lcov.info"
            ;;
        html)
            format_args="--html"
            ;;
        *)
            echo Invalid format
            exit 1
            ;;
    esac

    cargo llvm-cov report $format_args

install path:
    just gui/ install '{{ path }}'
    just cli/ install '{{ path }}'

uninstall path:
    just gui/ uninstall '{{ path }}'
    just cli/ uninstall '{{ path }}'

alias fmt := format
[parallel]
format: android::format cli::format gui::format i18n::format i18n-macros::format lib::format lib-macros::format lib-has::format format-docs

format-docs:
    #!/usr/bin/env bash
    if command -v prettier > /dev/null; then
        prettier --write README.md CHANGELOG.md ROADMAP.md docs/*.md
    else
        echo "Prettier not installed, skipping markdown formatting"
    fi

[parallel]
format-check: android::format-check cli::format-check gui::format-check i18n::format-check i18n-macros::format-check lib::format-check lib-macros::format-check lib-has::format-check format-check-docs

format-check-docs:
    #!/usr/bin/env bash
    if command -v prettier > /dev/null; then
        prettier --check README.md CHANGELOG.md ROADMAP.md docs/*.md
    else
        echo "Prettier not installed, skipping markdown format check"
    fi

shellcheck:
    {{fdfind}} --type file --extension sh --exec shellcheck {}
