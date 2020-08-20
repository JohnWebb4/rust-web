#! /usr/bin/env bash



function build {
  if [ -f 'site/opt.wasm' ]; then
    echo 'Cleaning wasm file'
    rm site/opt.wasm
  fi

  cargo build --target wasm32-unknown-unknown --release
  wasm-strip "target/wasm32-unknown-unknown/release/$1"
  wasm-opt -o "site/$2" -Oz "target/wasm32-unknown-unknown/release/$1"
}

function size_check {
  size=`ls -lh site | awk -F ' ' '{sum+=$5;} END{print sum}'`

  if [[ $size -gt 13000 ]]; then
    echo "Over 13 size limit"
  fi

  echo "Size is: $size B"
}

function main {
  build "rust_web.wasm" "opt.wasm"

  size_check
}

main