#!/bin/sh

set -e

if [ "$#" -lt 1 ]; then
    echo "Usage: ${0} <function_name>"
    exit 1
fi

FUNCTION_NAME=${1}

charon cargo --preset=aeneas \
  --start-from crate::rv64::${FUNCTION_NAME} \
  --opaque crate::wrappers \
  --dest-file ${FUNCTION_NAME}.llbc

aeneas -backend lean -split-files -dest lean/${FUNCTION_NAME} ${FUNCTION_NAME}.llbc
