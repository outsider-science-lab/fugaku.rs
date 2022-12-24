#! /bin/bash

source /vol0004/apps/oss/llvm-v14.0.1/init.sh

set -eux

(cd bed && cargo build --release)
