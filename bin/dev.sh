#! /usr/bin/env bash

./bin/build.sh || { echo 'Build failed'; exit 1; }

npx nodemon server.js