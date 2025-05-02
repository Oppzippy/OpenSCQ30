default:
    @just --choose

build profile='dev':
    just gui/ build '{{profile}}'
    just cli/ build '{{profile}}'
    just android/ build '{{profile}}'

test:
    just lib/ test
    just cli/ test
    just gui/ test
    just android/ test

test-cov:
    just lib/ test-cov
    just cli/ test-cov
    just gui/ test-cov
    just android/ test-cov

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
    just gui/ install '{{path}}'
    just cli/ install '{{path}}'

uninstall path:
    just gui/ uninstall '{{path}}'
    just cli/ uninstall '{{path}}'

alias fmt := format
format:
    -just android/ format
    -just cli/ format
    -just gui/ format
    -just i18n/ format
    -just i18n-macros/ format
    -just lib/ format

format-check:
    just android/ format-check
    just cli/ format-check
    just gui/ format-check
    just i18n/ format-check
    just i18n-macros/ format-check
    just lib/ format-check

