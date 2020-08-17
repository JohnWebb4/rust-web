#! /usr/bin/env bash

args="$@"
MAX_SIZE="13"

check_file_size() {
  FOLDER_SIZE=`du -h -d 1 dist | sed s/[^0-9]*//g`

  if [ "$FOLDER_SIZE" -ge "$MAX_SIZE" ]; then 
    echo "ERROR: File size is $FOLDER_SIZE K when max is $MAX_SIZE K"
    exit 1
  fi
}

main() {
  # Clean dist
  echo 'Cleaning dist'
  rm -r dist
  mkdir dist

  # Copy index.html
  echo 'Copying index.html'
  cp src/index.html dist/

  # Building webpack
  echo 'Packaging'
  if [[ "$args" == *"--watch"* ]]; then 
    webpack-dev-server
  else 
    npx webpack $args

    # Check file size
    check_file_size
  fi
}

main