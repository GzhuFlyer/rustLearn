#include<stdint.h>
#include<stdio.h>
#include<stdlib.h>
size_t test1(int a,void* b){
    printf("a=%d\n",a);
    if (NULL != b){
        printf("b = %s\n",(char*)b);
    }else{
        return -1;
    }
    return 0;
}