#! /bin/bash

source /vol0004/apps/oss/llvm-v14.0.1/init.sh

set -eux

env --chdir bed cargo build --release
