.PHONY: FORCE

.PHONY: all
all: build;

gen: FORCE
	./crates/mpi-sys/gen.sh

.PHONY: build
build: FORCE
	./build.sh

.PHONY: build-release
build-release: FORCE
	./build.sh --release

.PHONY: run
run: build-release
	pjsub job.sh
	pjstat

.PHONY: ps
ps: FORCE
	@pjstat

.PHONY: clean
clean: FORCE
	rm -rf ./target/
	rm -rf job.sh.*.err job.sh.*.out ./output.* ./*.stats
