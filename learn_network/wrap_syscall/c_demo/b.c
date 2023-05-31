#include <libsyscall_intercept_hook_point.h>
#include <syscall.h>
#include <errno.h>
#include <stdio.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <string.h>

// $ cc a.c -lsyscall_intercept -fpic -shared -g -o a.so
// $ LD_LIBRARY_PATH=. LD_PRELOAD=./a.so ls
__attribute__((constructor)) void
myselfinit(void)
{
	// printf("hello");
	// Set up the callback function
	// intercept_hook_point = hook;
    printf("good\n");
}

//cc b.c -fpic -shared -g -o b.so