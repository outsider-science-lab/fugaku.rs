#! /bin/bash
cd "$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
source "/vol0004/apps/oss/spack/share/spack/setup-env.sh"
spack load /pvhwhce # load LLVM 12

set -eux
# TO INSTALL:
#   cargo install bindgen-cli

compiler_dir="$(cd "$(dirname "$(which mpifccpx)")" && cd .. && pwd)"

rm -f src/ffi.rs
echo "#![allow(warnings)]" > src/ffi.rs
bindgen "${compiler_dir}/include/mpi/fujitsu/mpi.h" >> src/ffi.rs
