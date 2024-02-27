#! /bin/bash
cd "$(cd "$(dirname "$(readlink -f "$0")")" && pwd)"

source /vol0004/apps/oss/llvm-v15.0.3/init.sh

set -eux

env --chdir bed cargo build --release
