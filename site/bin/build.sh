#! /usr/bin/env bash

args="$@"

# Clean dist
echo 'Cleaning dist'
rm -r dist
mkdir dist

# Copy index.html
echo 'Copying index.html'
cp src/index.html dist/

# Building webpack
echo 'Packaging'
npx webpack $args