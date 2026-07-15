#!/bin/sh

PATH="/home/gurvan/git/uni/phd/verif-riscv/kani-dev/scripts:${PATH}"
# Important options:
# --exact: Just do the specified harness
# --only-codegen
# --output-format terse
RUSTFLAGS="--emit mir" cargo kani -Z const-prop-prune --harness=check_memset2
