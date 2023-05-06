#include<stdio.h>
#include <stdarg.h>


#define myprintf(...)   ({printf(__VA_ARGS__);})

void my_printf(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    printf(fmt, args);
    va_end(args);
}

int main()
{
    int a = 1;
    float b = 1.00;
    myprintf("hello wrold,a=%d,b = %f\n",a,b);
    my_printf("hello wrold,a=%d,b = %f\n",a,b);
    printf("hello wrold,a=%d,b = %f\n",a,b);
    return 0;
}