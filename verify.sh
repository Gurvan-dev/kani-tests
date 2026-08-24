#!/bin/sh

set -e

BASE_DIR="$(realpath "$(dirname "${0}")")"
RES_DIR="${BASE_DIR}/results"
KANI_DEV_DIR="${BASE_DIR}/kani-dev"
PATH="${KANI_DEV_DIR}/scripts:${KANI_DEV_DIR}/target/debug:${KANI_DEV_DIR}/target/debug/deps:${KANI_DEV_DIR}/build/bin:${PATH}"

show_help() {
    echo "Usage: ${0} <target> <harness1> [harness2 ...]"
    echo "Targets:"
    echo "    smt         Dump cbmc smt queries"
    echo "    check       Check options"
}

if [ "$#" -lt 2 ]; then
    show_help
    exit 1
fi

TARGET="${1}"
shift
KANI_OPT="-Z unstable-options -Z const-prop-prune"

mkdir -p "${RES_DIR}"

for HARNESS in "$@"; do
    echo "# Running harness '${HARNESS}'"
    case "${TARGET}" in
        check)
            TIME_FILE="${RES_DIR}/${HARNESS}.time"
            LOG_FILE="${RES_DIR}/${HARNESS}.log"

            command time -p sh -c \
                "( RUSTFLAGS=\"--emit mir\" cargo kani ${KANI_OPT} --harness \"${HARNESS}\" ) > \"${LOG_FILE}\" 2>&1" \
                2> "${TIME_FILE}"

            echo "# Output saved to ${LOG_FILE}"
            echo "# Timing saved to ${TIME_FILE}"
            ;;

        smt)
            OUT_FILE="${RES_DIR}/${HARNESS}.smt2"
            cargo kani ${KANI_OPT} --harness "${HARNESS}" --output-format old --cbmc-args --smt2 --outfile "${OUT_FILE}"
            ;;
        *)
            echo "Error: Unknown target '${TARGET}'" >&2
            show_help
            exit 1
            ;;
    esac
done
