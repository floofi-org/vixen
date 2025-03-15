#!/usr/bin/env sh

pushd $(dirname $0) > /dev/null

cargo run -p vasm $1 rom.bin &&
cargo run -p vemu rom.bin

popd
