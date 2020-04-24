#!/usr/bin/env bash

function info () {
    echo -e "[\033[0;34m $@ \033[0m]"
}

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
CURRENT=$(pwd)

# Shrinking wasm size
# binaryen 
# `brew install binaryen`
# @see https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size

# wasm-bindgen
# `cargo install wasm-bingen-cli`


# client build
cd ${SRCDIR}/frontendyew

info "Building rust for wasm"
cargo build --target wasm32-unknown-unknown --release
# cargo web deploy --target=wasm32-unknown-unknown --release
OUTDIR=${SRCDIR}/backend/static/
BUILDDIR=${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/

info "wasm-bindgen"
wasm-bindgen --target web --no-typescript --out-dir $OUTDIR --out-name frontendyew $BUILDDIR/frontendyew.wasm

info Optitimization
#`cargo install wasm-gc`
# ───┬───────────────────────────────────────────────────┬──────┬──────────┬─────────────
#  # │ name                                              │ type │ size     │ modified 
# ───┼───────────────────────────────────────────────────┼──────┼──────────┼─────────────
#  0 │ actixcomplex/backend/static/frontendyew.html      │ File │    509 B │ 1 hour ago 
#  1 │ actixcomplex/backend/static/frontendyew.js        │ File │  19.3 KB │ 15 secs ago 
#  2 │ actixcomplex/backend/static/frontendyew_bg.wasm   │ File │ 277.8 KB │ 15 secs ago 
#  3 │ actixcomplex/backend/static/frontendyew_orig.wasm │ File │ 354.2 KB │ 15 secs ago 
# ───┴───────────────────────────────────────────────────┴──────┴──────────┴─────────────
cd ${SRCDIR}/backend/static/
mv frontendyew_bg.wasm frontendyew_orig.wasm
# wasm-gc frontendyew_orig.wasm frontendyew_gc.wasm
wasm-opt frontendyew_orig.wasm -Os -o frontendyew_bg.wasm

# server build
cd ${SRCDIR}

mkdir -p backend/static

#cp $BUILDDIR/frontendyew.js ${SRCDIR}/backend/static/
#cp $BUILDDIR/frontendyew.wasm ${SRCDIR}/backend/static/
# wasm-opt -Oz -o ${SRCDIR}/backend/static/frontendyew.wasm $BUILDDIR/frontendyew.wasm

# Gziping 
# gzip cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm
# mv cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm.gz cd ${SRCDIR}/frontendyew/backend/static/frontendyew.wasm

cd ${SRCDIR}/..
cargo run --bin actixcomplex # --release
