.PHONY: test
test:
	test/test.sh

.PHONY: testrun
testrun:
	cat $(file)/in.txt | cargo run -- `cat $(file)/option.txt` "`cat $(file)/schema.txt`"
