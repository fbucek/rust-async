#!/usr/bin/env bash

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
CURRENT=$(pwd)

# Shrinking wasm size
# binaryen 
# `brew install binaryen`
# @see https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size

# client build
cd ${SRCDIR}/frontendyew

cargo web deploy --target=wasm32-unknown-unknown --release

# server build
cd ${SRCDIR}

mkdir -p backend/static

cp ${CARGO_TARGET_DIR}/deploy/frontendyew.* ${SRCDIR}/backend/static/
wasm-opt -Oz -o ${SRCDIR}/backend/static/frontendyew.wasm ${CARGO_TARGET_DIR}/deploy/frontendyew.wasm 

# Gziping 
# gzip cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm
# mv cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm.gz cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm

cd ${SRCDIR}/..
cargo run --bin actixcomplex # --release
