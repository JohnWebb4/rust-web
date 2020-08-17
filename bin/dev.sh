#! /usr/bin/env bash

./bin/build.sh

pushd site
npm run serve
popd site;