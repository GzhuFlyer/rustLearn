#include<stdio.h>
#include<time.h>
#include<stdlib.h>
int main()
{
    printf("now time is %ld\n",time(NULL));
    printf("hello world\n");
    sleep(10);
    exit(0);
    return 0;
}