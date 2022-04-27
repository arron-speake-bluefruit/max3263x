#!/bin/sh

set -e

rm src/ build.rs device.x -rf
svd2rust -i svd/max3263x.svd -o .
form -i lib.rs -o src/
rm lib.rs
cargo fmt
