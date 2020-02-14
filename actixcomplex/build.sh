#!/usr/bin/env bash

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
CURRENT=$(pwd)

# Shrinking wasm size
# binaryen 
# `brew install binaryen`
# @see https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size

# client build
cd ${SRCDIR}/frontend

cargo web deploy --target=wasm32-unknown-unknown --release

# server build
cd ${SRCDIR}

mkdir -p backend/static

cp ${CARGO_TARGET_DIR}/deploy/frontend.* ${SRCDIR}/backend/static/
wasm-opt -Oz -o ${SRCDIR}/backend/static/frontend.wasm ${CARGO_TARGET_DIR}/deploy/frontend.wasm 

# Gziping 
# gzip cd ${SRCDIR}/frontend/backend/static/frontend.wasm
# mv cd ${SRCDIR}/frontend/backend/static/frontend.wasm.gz cd ${SRCDIR}/frontend/backend/static/frontend.wasm

cd ${SRCDIR}/..
cargo run --bin actixcomplex
