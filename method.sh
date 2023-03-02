#!/bin/sh

kitty_cargo() {
  executable="kitties"
  if ! command -v "$executable" >/dev/null 2>&1; then
    echo "kitties not found, falling back on kitty"
    executable="kitty"
  fi
  if ! command -v "$executable" >/dev/null 2>&1; then
    echo "kitty not found, exiting"
    return 1
  fi

  if [[ $# -ne 1 ]]; then
    echo "usage: kitty_cargo <id>"
    echo "       where <id> is the ID for the Kattis problem"
    exit 1
  fi

  # Create the crate and move into it
  cargo new $1
  cd $1

  mkdir tmp && cd tmp
  # Use kitty to download the test files and move them into the create folder
  "$executable" get $1 --lang rs
  rm -f $1.rs || true


  mv $1/test ..
  # Remove the kitty generated folder - we don't need it
  cd ..
  rm -rf $1 && rm -rf tmp

  # Create a symlink to the main source file so that testing and submission will work
  ln src/main.rs $1.rs
}

kitty_cargo $1

