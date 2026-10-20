#!/bin/sh

MEMSET_NAME=memset4

charon cargo --preset=aeneas \
  --start-from crate::rv64::${MEMSET_NAME} \
  --opaque crate::wrappers \
  --dest-file ${MEMSET_NAME}.llbc
aeneas -backend lean -split-files -dest lean/MemsetCore ${MEMSET_NAME}.llbc
