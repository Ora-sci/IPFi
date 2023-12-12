#!/bin/bash
set -e

cd "`dirname $0`"/../ip-nft
cargo build --all --target wasm32-unknown-unknown --release

cd ..
cd "`dirname $0`"/../ip-token
cargo build --all --target wasm32-unknown-unknown --release

cd ..
cd "`dirname $0`"/../ip-marketplace
cargo build --all --target wasm32-unknown-unknown --release

cd ..
cp ./target/wasm32-unknown-unknown/release/*.wasm ./out/