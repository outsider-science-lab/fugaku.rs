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
