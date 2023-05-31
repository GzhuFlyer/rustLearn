#include"c.h"
#include<stdio.h>

int (*my_point)(int a,int b);

float my_callback(int arg)
{
    return arg / 2.0f;
}

//CallbackFunc x float call_callback(CallbackFunc x, int arg)
float call_callback( float (*x)(int), int arg)
{
    if (my_point !=NULL){
        my_point(arg,arg+1);
        printf("call_callback(): my_point is not null,arg = %d\n",arg);
    }

    arg += 10;
    return x(arg);
}