.PHONY: FORCE

.PHONY: all

all: FORCE
	./build.sh

gen: FORCE
	./crates/mpi-sys/gen.sh

.PHONY: build
build: all;

.PHONY: run
run: build
	pjsub job.sh
	pjstat

.PHONY: ps
ps: FORCE
	@pjstat

.PHONY: clean
clean: FORCE
	rm -rf ./target/
	rm -rf job.sh.*.err job.sh.*.out ./output.*
