#! /usr/bin/env bash

pushd pkg
wasm-pack build
npm link
popd

pushd site
npm link rust-web
npm i
popd