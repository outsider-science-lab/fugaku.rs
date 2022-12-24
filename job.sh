#!/bin/sh
#PJM -L "node=16"
#PJM -L "rscgrp=small"
#PJM -L "elapse=10:00"
#PJM -g hp220271
#PJM -x PJM_LLIO_GFSCACHE=/vol0004

# See https://www.fugaku.r-ccs.riken.jp/doc_root/en/user_guides/FugakuSpackGuide/intro.html#sourcing-environment-script
#. /vol0004/apps/oss/spack/share/spack/setup-env.sh
#spack load XXXX
# See https://www.fugaku.r-ccs.riken.jp/doc_root/en/user_guides/FugakuSpackGuide/intro.html#path-of-dynamic-link-libraries-of-the-operating-system
#export LD_LIBRARY_PATH=/lib64:$LD_LIBRARY_PATH

# See https://www.fugaku.r-ccs.riken.jp/doc_root/en/user_guides/use_latest/LyeredStorageAndLLIO/TheSecondLayerStrage.html#common-file-distribution-function-llio-transfer
#/usr/bin/llio_transfer <path_to_file>

source /vol0004/apps/oss/llvm-v14.0.1/init.sh

mpiexec ./target/aarch64-unknown-linux-gnu/release/bed
