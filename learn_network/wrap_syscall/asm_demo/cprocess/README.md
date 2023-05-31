#预处理
gcc -E -I./inc test.c -o test.i
或者 cpp test.c -I./inc -o test.i
#编译
gcc -S -I./inc test.c -o test.s     gcc -S mymath.c -o mymath.s
#汇编
as test.s -o test.o 
gcc -c test.s -o test.o gcc -c mymath.s -o mymath.o
#编译