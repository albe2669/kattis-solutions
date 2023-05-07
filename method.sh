#!/bin/bash
# All credit for the original version of this script goes to https://github.com/andreaswachs/Kattis and his kitty_cargo tool.

kitty_cargo() {
  executable="kitty"
  if ! command -v "$executable" >/dev/null 2>&1; then
    echo "kitty not found, exiting"
    return 1
  fi

  if [[ $# -ne 1 ]]; then
    echo "usage: $0 <id>"
    echo "       where <id> is the ID for the Kattis problem"
    exit 1
  fi

  # Create the crate and move into it
  cargo new $1
  if [[ ! -d $1 ]]; then
    exit 1
  fi
  cd $1

  problem_root=$PWD

  mkdir tmp && cd tmp

  # Use kitty to download the test files and move them into the created folder
  "$executable" get $1 --lang rs
  if [[ ! -d $1 ]]; then
    exit 1
  fi
  cd $1
  
  # pr/tmp/pr
  if [[ -f $1.rs ]]; then
    mv $1.rs $problem_root/src/main.rs
  fi
  
  if [[ -d test ]]; then
    mv test $problem_root
  fi

  cd $problem_root
  rm -rf tmp

  ln src/main.rs $1.rs
  echo "$1.rs" >> .gitignore
  echo "$1" >> .gitignore
  echo "target/" >> .gitignore
}

kitty_cargo $1

