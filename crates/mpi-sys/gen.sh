#! /bin/bash

# TO INSTALL:
#   cargo install bindgen-cli

cd "$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
source "/vol0004/apps/oss/spack/share/spack/setup-env.sh"

spack load /cysddka || false

set -eux

COMPILER_DIR="$(cd "$(dirname "$(which mpifccpx)")" && cd .. && pwd)"

rm -f src/ffi.rs
echo "#![allow(warnings)]" > src/ffi.rs
bindgen "${COMPILER_DIR}/include/mpi/fujitsu/mpi.h" >> src/ffi.rs
