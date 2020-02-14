#!/usr/bin/env bash

CURRENT=$(pwd)

# Shrinking wasm size
# binaryen 
# `brew install binaryen`
# @see https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size

# client build
cd frontend

cargo web deploy --target=wasm32-unknown-unknown --release

# server build
cd $CURRENT

mkdir -p backend/static

cp ${CARGO_TARGET_DIR}/deploy/frontend.* $CURRENT/backend/static/
wasm-opt -Oz -o $CURRENT/backend/static/frontend.wasm ${CARGO_TARGET_DIR}/deploy/frontend.wasm 

# Gziping 
# gzip $CURRENT/backend/static/frontend.wasm
# mv $CURRENT/backend/static/frontend.wasm.gz $CURRENT/backend/static/frontend.wasm

cd $CURRENT/..
cargo run --bin actixcomplex
