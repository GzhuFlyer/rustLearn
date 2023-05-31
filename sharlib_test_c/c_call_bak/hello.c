#include<stdio.h>
#include "c.h"

void test1()
{
	printf("hello\n");
}
void test2(void (*p)())
{
	(*p)();
}

float my_callback_2(int arg)
{
    return arg / 2.0f;
}

float my_callback_3(int arg)
{
    return arg / 3.0f;
}

void my_run(){
    CallbackFunc tFun = NULL;
    tFun = my_callback_3;
    float result = call_callback(tFun, 30);
    printf("Result: %f\n", result);
}

int my_point_fun(int x,int y){
    int z = x + y;
    printf("z = %d\n",z);
}

int main()
{
	test2(test1);
  
    // float result = call_callback(my_callback_2, 10);
    // printf("Result: %f\n", result);
    // my_run();
    // Run myrun = my_run;
    // my_point = my_point_fun;
     float result = call_callback(my_callback_2, 10);
    printf("Result: %f\n", result);
    // CallbackFunc_plus = call_callback(tFun,30);

	return 0;
}
// gcc -shared -fPIC -o c.so c.c
// LD_LIBRARY_PATH=$(pwd) ./hello
// gcc -o hello hello.c c.so 
