RUSTC = rustc --opt-level=3 --out-dir bin/

.PHONY : risp clean test benchmark help

help:
	@echo "--- risp (0.000000000000000000000000000000000000011755)" \
	&& echo "make run        - Builds executable and runs it" \
	&& echo "make risp       - Builds executable" \
	&& echo "make test       - Builds and executes tests" \
	&& echo "make test-bench - Builds and executes tests and benchmarks" \
	&& echo "make clean      - Deletes outputs"

run  : risp
	cd "bin/" && ./risp

risp :
	$(RUSTC) src/main.rs

test :
	$(RUSTC) --test src/main.rs
	cd "bin/" && ./risp

test-bench :
	$(RUSTC) --test src/main.rs
	cd "bin" && ./risp --bench --save-metrics=bench.json

clean :
	rm -rf bin/*
