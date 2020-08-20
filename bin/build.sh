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
  ## List file sizes
  ls -l site | awk -F ' ' '{print $9, ": ", $5, "B";}'

  size=`ls -l site | awk -F ' ' '{sum+=$5;} END{print sum}'`

  echo "Size is: $( bc <<< "scale=2;$size / 1000" ) K"

  if [[ $size -gt 13000 ]]; then
    echo "Over 13K size limit"
    exit 1
  fi
}

function main {
  build "rust_web.wasm" "opt.wasm"

  size_check
}

main
