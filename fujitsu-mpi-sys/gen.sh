#! /bin/bash
cd "$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"
source "/vol0004/apps/oss/spack/share/spack/setup-env.sh"
spack load /pvhwhce # load LLVM 12

set -eux
# TO INSTALL:
#   cargo install bindgen-cli
bindgen /opt/FJSVxtclanga/tcsds-1.2.36/include/mpi/fujitsu/mpi.h -o src/ffi.rs
