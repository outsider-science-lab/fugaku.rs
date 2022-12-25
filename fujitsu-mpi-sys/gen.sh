#! /bin/bash
source "/vol0004/apps/oss/spack/share/spack/setup-env.sh"
spack load /pvhwhce # load LLVM 12
bindgen /opt/FJSVxtclanga/tcsds-1.2.36/include/mpi/fujitsu/mpi.h -o src/mpi.rs
