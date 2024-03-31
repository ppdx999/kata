#!/bin/bash

root_dir=$(dirname $(cd $(dirname $0) && pwd))
test_dir=$root_dir/test
cmd=$root_dir/target/debug/schematch

tmp_stdout=$(mktemp)
tmp_stderr=$(mktemp)
trap 'rm -f $tmp_stdout $tmp_stderr' EXIT

# Build the project
$(cd $root_dir && cargo build > /dev/null 2>&1)

test_fail() {
  exit 1
}

assert() {
	result=$?
  if [ $result -ne $(cat $1/result.txt) ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected exit code: $(cat $1/result.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual exit code  : $result \033[0m\n"
    exit 1
  fi

  if [ "$(cat $1/out.txt | tr -d '[:blank:]' )" != "$(cat $tmp_stdout | tr -d '[:blank:]' )" ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected stdout: \n$(cat $1/out.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual stdout  : \n$(cat $tmp_stdout) \033[0m\n"
    exit 1
  fi

  if [ "$(cat $1/err.txt | tr -d '[:blank:]' )" != "$(cat $tmp_stderr | tr -d '[:blank:]' )" ]; then
    printf "\033[1;31mTest $1 failed\033[0m\n"
    echo
    printf "\033[1;31mExpected stderr: \n$(cat $1/err.txt)\033[0m\n"
    echo
    printf "\033[1;31mActual stderr  : \n$(cat $tmp_stderr) \033[0m\n"
    exit 1
  fi
}

run_test() {
  echo "Running test $1"

  #
  # Arrange & Act
  # 
  cat $1/in.txt | $cmd \
    $(cat $1/option.txt) \
    "$(cat $1/schema.txt)" > $tmp_stdout 2> $tmp_stderr

  #
  # Assert
  #
  assert $1
}

run_test $test_dir/tsv/primitive/integer
run_test $test_dir/tsv/primitive/integer_dont_accept_float
run_test $test_dir/tsv/primitive/string
run_test $test_dir/tsv/primitive/boolean_true
run_test $test_dir/tsv/primitive/boolean_false
run_test $test_dir/tsv/primitive/boolean_only_accept_true_or_false
run_test $test_dir/tsv/primitive/float
run_test $test_dir/tsv/primitive/float_accept_integer
run_test $test_dir/tsv/primitive/float_invalid
run_test $test_dir/tsv/primitive/null
run_test $test_dir/tsv/multi_term_multi_column
run_test $test_dir/tsv/sum_type
run_test $test_dir/tsv/multiline_schema

run_test $test_dir/tsv/multi_data_type_mismatch_error
run_test $test_dir/tsv/field_number_mismatch_error

run_test $test_dir/json/primitive/string
run_test $test_dir/json/primitive/string_invalid
run_test $test_dir/json/primitive/number
run_test $test_dir/json/primitive/boolean
run_test $test_dir/json/primitive/boolean_invalid
run_test $test_dir/json/primitive/null

run_test $test_dir/json/empty
run_test $test_dir/json/multi_property
run_test $test_dir/json/nested_object

run_test $test_dir/json/top_level_type

# Array
run_test $test_dir/json/array/string
run_test $test_dir/json/array/object
run_test $test_dir/json/array/data_type_mismatch_error

# all
run_test $test_dir/json/all

run_test $test_dir/json/property_not_found_error

echo "Running test without option"
cat $test_dir/tsv/primitive/integer/in.txt | $cmd "$(cat $test_dir/tsv/primitive/integer/schema.txt)" > $tmp_stdout 2> $tmp_stderr
assert $test_dir/tsv/primitive/integer


printf "\033[1;32mAll tests passed\033[0m\n"
