#include <libsyscall_intercept_hook_point.h>
#include <syscall.h>
#include <errno.h>
#include <stdio.h>

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#include <string.h>
// int reentrance_guard_flag;

int self_op()
{
	int ret = open("./intercept.txt",O_CREAT|O_TRUNC,0666);
	// reentrance_guard_flag = 0;
	return ret;
}

static int
hook(long syscall_number,
			long arg0, long arg1,
			long arg2, long arg3,
			long arg4, long arg5,
			long *result)
{
	// if (reentrance_guard_flag)
	// 	return 1;
	// reentrance_guard_flag = 1;
	if (syscall_number == SYS_getdents) {
		/*
		 * Prevent the application from
		 * using the getdents syscall. From
		 * the point of view of the calling
		 * process, it is as if the kernel
		 * would return the ENOTSUP error
		 * code from the syscall.
		 */
		// return 1;
		*result = -ENOTSUP;
		return 0;
	} else {
		if (syscall_number == SYS_open){
			*result = -ENOTSUP;
			return 0;
			// printf("(const char *)arg0, = %s\n",(const char *)arg0);
			int cmp = strcmp((const char *)arg0,"hello");
			// printf("cmp = %d\n",cmp);
			if (cmp == 0){
				int ret = self_op();
				return 0;
			}
		}
		/*
		 * Ignore any other syscalls
		 * i.e.: pass them on to the kernel
		 * as would normally happen.
		 */
		return 1;
	}
}
// $ cc a.c -lsyscall_intercept -fpic -shared -g -o a.so
// $ LD_LIBRARY_PATH=. LD_PRELOAD=./a.so ls
static __attribute__((constructor)) void
myselfinit(void)
{
	// printf("hello");
	// Set up the callback function
	intercept_hook_point = hook;
}