#! /bin/bash

ROOT_PATH="$(cd "$(realpath "$(dirname "$0")")" && pwd)"
cd "${ROOT_PATH}"

source '/vol0004/apps/oss/llvm-v15.0.3/init.sh'

set -eux

env --chdir bed cargo build "$@"
