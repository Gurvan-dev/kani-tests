#!/bin/sh

set -e

# This is hardcoded for now but could be configured
PATH="/home/gurvan/git/uni/phd/verif-riscv/kani-dev/scripts:${PATH}"

# NOTE(Useful kani options):
# --exact                       Just do the specified harness
# --only-codegen                It's in the name
# --output-format terse         Print only useful generation

show_help() {
    echo "Usage: ${0} [target] [harness]"
    echo "Targets:"
    echo "    smt         Dump cbmc smt queries"
    echo "    check       Check options"
}

TARGET="${1}"
KANI_OPT="-Z unstable-options -Z const-prop-prune"
if [ -n "${2}" ]; then
    KANI_OPT="${KANI_OPT} --harness=${2}"
fi

case "${TARGET}" in
    check)
        RUSTFLAGS="--emit mir" cargo kani ${KANI_OPT} --harness=kani_minimal_memset2
        ;;

    smt)
        cargo kani ${KANI_OPT} --output-format old --cbmc-args --smt2 --outfile query.smt2
        ;;
    *)
        echo "Error: Unknown target '${TARGET}'" >&2
        show_help
        exit 1
        ;;
esac
