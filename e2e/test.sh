#!/bin/bash

root_dir=$(dirname $(cd $(dirname $0) && pwd))
test_dir=$root_dir/e2e

run_test() {
  echo "Running test $1"
  cat $1/in.txt | cargo run -- $(cat $1/option.txt) "$(cat $1/schema.txt)" > /dev/null 2>&1
  [ $? -eq $(cat $1/result.txt) ] || (echo "Test $1 failed" && exit 1)
}

run_test $test_dir/tsv/multi_column

printf "\033[1;32mAll tests passed\033[0m\n"
