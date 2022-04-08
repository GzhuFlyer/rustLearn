// #include<stdio.h>

// int main()
// {
//     // unsigned int a = (void *)0xdeadc0de;
//     unsigned int a = (void *)0x12345678;
//     // for (int i=0;i<100;i++)
//     // {
//     //     printf("a=%d\n",*(a++));
//     // }
//     printf("%0x\n",a);
//     printf("%0x\n",*((char*)a));
//     return 0;
// }

// #include <stdio.h>
// #include <stdlib.h>

// #if !defined(__x86_64) && !defined(__i386)
// #error("Unsupported Architecture")
// #endif

// int cast_c(float a)
// {
//     return (int) a;
// }

// int cast_asm(float a)
// {
//     (void) a;

//     asm(".intel_syntax noprefix\n");
//     asm("xor eax,eax\n"
// #ifdef __x86_64
//         "movss  xmm0,DWORD PTR [rbp-0x4]\n"
// #else
//         "movss  xmm0,DWORD PTR [ebp+0x8]\n"
// #endif
//         "cvttss2si eax,xmm0\n");
// #ifdef __x86_64
//     asm("pop rbp\nret\n");
// #else
//     asm("pop ebp\nret\n");
// #endif
//     asm(".att_syntax noprefix\n");

//     return 0xdeadc0de;
// }

// int main(int argc, char **argv)
// {
//     (void) argc;
//     (void) argv;

//     float f __attribute__((aligned(0x4))) = 100.5f;
//     printf("cast_c..: %d\n", cast_c(f));
//     printf("cast_asm: %d\n", cast_asm(f));
// }
#include <unistd.h>
#include <sys/types.h>
#include<stdio.h>

int main()
{
    printf("main\n");
    // if(0==truncate("/home/fzw/workspace/bky/rustLearn/burstfs_daemon.log",100)){
    if(0==truncate((void *)0xdeadc0de,100)){
        printf("ok\n");
    }else{
        printf("error\n");
    }
    return 0;
}