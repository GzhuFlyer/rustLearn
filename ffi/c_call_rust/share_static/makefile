GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

share: clean cargo
	mkdir cbin
	$(GCC_BIN) -o ./cbin/main ./c_src/main.c -I./src -L$(pwd)/target/debug -lcallrust
	LD_LIBRARY_PATH=./target/debug ./cbin/main 

static: clean cargo
	mkdir cbin
	$(GCC_BIN) -o ./cbin/main ./c_src/main.c -I./src ./target/debug/libcallrust.a -lpthread -ldl -static
	./cbin/main
# all:
# 	echo "hello"
# 	echo $(GCC_BIN)
# 	echo $(CARGO_BIN)

clean:
	$(CARGO_BIN) clean
	rm -rf ./cbin	

cargo:
	$(CARGO_BIN) build
