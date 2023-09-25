SRC_DIR := src

EXECUTABLE_NAME := kengen
DEV_EXECUTABLE := target/debug/$(EXECUTABLE_NAME)

$(DEV_EXECUTABLE):
	cargo build

build: $(DEV_EXECUTABLE)

go:
	cargo run

execdbg: $(DEV_EXECUTABLE)
	./$(DEV_EXECUTABLE)

rundbg:
	cargo run -- -l debug

runinfo:
	cargo run -- -l info

runcrit:
	cargo run -- -l critical

kill:
	pkill --signal=9 $(EXECUTABLE_NAME)