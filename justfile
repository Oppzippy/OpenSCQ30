default:
    @just --choose

build profile='dev':
    just gui/ build '{{profile}}'
    just cli/ build '{{profile}}'
    just android/ build '{{profile}}'
    just web/ build '{{profile}}'

test:
    just gui/ test
    just cli/ test
    just android/ test
    just web/ test

test-cov:
    just gui/ test-cov
    just cli/ test-cov
    just android/ test-cov
    just web/ test-cov

llvm-cov-clean:
    cargo llvm-cov clean --workspace

test-cov-report format='lcov':
    #!/usr/bin/env bash
    set -eou pipefail

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

install prefix:
    just gui/ install '{{prefix}}'
    just cli/ install '{{prefix}}'

uninstall prefix:
    just gui/ uninstall '{{prefix}}'
    just cli/ uninstall '{{prefix}}'

format:
    just android/ format
    just cli/ format
    just gui/ format
    just lib/ format
    just lib_protobuf/ format
    just web/ format

format-check:
    just android/ format-check
    just cli/ format-check
    just gui/ format-check
    just lib/ format-check
    just lib_protobuf/ format-check
    just web/ format-check

