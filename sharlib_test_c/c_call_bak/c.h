#include <stddef.h>

typedef float (*CallbackFunc)(int);
//float call_callback(CallbackFunc callback, int arg);

float call_callback( float (*callback)(int) , int arg);

typedef float (*CallbackFunc_plus)(CallbackFunc,int);

CallbackFunc_plus  g_run = NULL;

typedef void (*Run)();

/*
extern int (*intercept_hook_point)(long syscall_number,
			long arg0, long arg1,
			long arg2, long arg3,
			long arg4, long arg5,
			long *result);

*/
extern int (*my_point)(int a,int b);