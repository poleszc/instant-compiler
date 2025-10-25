PROJECT_NAME=instant-compiler

JASMIN = java -jar lib/jasmin.jar
LLVM-AS = llvm-as
LLI = LLI

all: build-all
	
build-all: build-jvm build-llvm

build-jvm:
	cargo build --bin insc_jvm --release
	cp target/release/insc_jvm

build-llvm:
	cargo build --bin insc_llvm --release
	cp target/release/insc_jvm

test: build
	@echo "Running tests..."

clean:
	echo "Cleaning up..."
	cargo clean
	rm -rf insc_jvm insc_llvm
	rm -rf tests/good/*.j tests/good/*.class tests/good/*.ll tests/good/*.bc

.PHONY:
	all build build-jvm build-llvm test clean