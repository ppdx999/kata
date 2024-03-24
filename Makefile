.PHONY: test
test:
	e2e/test.sh

.PHONY: testrun
testrun:
	cat $(file)/in.txt | cargo run -- `cat $(file)/option.txt` "`cat $(file)/schema.txt`"
