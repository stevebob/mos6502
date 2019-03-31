#!/bin/bash
set -euo pipefail
cd "$( dirname "${BASH_SOURCE[0]}" )"
./build_sample.sh $1 | cargo run --manifest-path=nes-emulator/Cargo.toml
