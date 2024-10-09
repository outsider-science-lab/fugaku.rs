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
run: build-release clean-jobs
	pjsub job.sh
	pjstat

.PHONY: ps
ps: FORCE
	@pjstat

.PHONY: clean-jobs
clean-jobs: FORCE
	rm -rf job.sh.*.err job.sh.*.out ./output.* ./*.stats ./.sdtin.*

.PHONY: clean-build
clean-build: FORCE
	rm -rf ./target/

.PHONY: clean
clean: clean-jobs

.PHONY: clean-all
clean-all: clean clean-build
