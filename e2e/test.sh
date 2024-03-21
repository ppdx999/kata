#!/bin/bash

root_dir=$(dirname $(cd $(dirname $0) && pwd))
test_dir=$root_dir/e2e

tmp_stdout=$(mktemp)
tmp_stderr=$(mktemp)
trap 'rm -f $tmp_stdout $tmp_stderr' EXIT

# Build the project
$(cd $root_dir && cargo build > /dev/null 2>&1)

test_fail() {
  exit 1
}

run_test() {
  echo "Running test $1"

  #
  # Arrange & Act
  # 
  cat $1/in.txt | $root_dir/target/debug/schematch \
    $(cat $1/option.txt) \
    "$(cat $1/schema.txt)" > $tmp_stdout 2> $tmp_stderr

  #
  # Assert
  #
  if [ $? -ne $(cat $1/result.txt) ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected exit code: $(cat $1/result.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual exit code  : $? \033[0m\n"
    exit 1
  fi

  if [ "$(cat $1/out.txt)" != "$(cat $tmp_stdout)" ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected stdout: \n$(cat $1/out.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual stdout  : \n$(cat $tmp_stdout) \033[0m\n"
    exit 1
  fi

  if [ "$(cat $1/err.txt)" != "$(cat $tmp_stderr)" ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected stderr: \n$(cat $1/err.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual stderr  : \n$(cat $tmp_stderr) \033[0m\n"
    exit 1
  fi
}

run_test $test_dir/tsv/multi_column

printf "\033[1;32mAll tests passed\033[0m\n"
