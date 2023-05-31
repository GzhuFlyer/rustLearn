#include<stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>


int main()
{
    int ret = open("hello",O_CREAT|O_TRUNC,0666);
    perror("open");
	return 0;
}

