#include<stdio.h>

int main()
{
    int a = -1;
    printf("a change = %lx\n",(unsigned int) a);
    unsigned int b = 2;
    unsigned int c = (int)a  + b;
    printf("c = %d\n",c);
}