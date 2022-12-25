.PHONY: FORCE

.PHONY: all

all: FORCE
	bash ./build.sh

.PHONY: build
build: all;

.PHONY: run
run: FORCE
	pjsub job.sh
	pjstat

.PHONY: clean
clean: FORCE
	rm -rf ./target/
	rm -rf job.sh.*.err job.sh.*.out ./output.*
