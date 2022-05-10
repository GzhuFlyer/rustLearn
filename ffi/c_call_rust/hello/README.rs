# c语言 调用 rust的库

vim call_rust.c
gcc call_rust.c -o call_rust -ldemo2 -L./target/debug
LD_LIBRARY_PATH=./target/debug ./call_rust