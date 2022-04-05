#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define MAXBUFSIZE 256

void main()
{
    char current_absolute_path[ MAXBUFSIZE ]; 
    int count; 
    count = readlink( "/proc/self/exe", current_absolute_path, MAXBUFSIZE ); 
    if ( count < 0 || count >= MAXBUFSIZE ) { 
    printf( "Failed\n" ); 
    return( EXIT_FAILURE ); 
    } 
 
    current_absolute_path[ count ] = '\0'; 
    printf( "/proc/self/exe -> [%s]\n", current_absolute_path ); 
      /* 从终端读取要执行的命令 */
    printf( "star main\n" );
          printf("exec time is %ld\n",time(NULL));
      if ( fork() == 0 ) {/* 子进程执行此命令 */
         execlp( "/home/gzhuflyer/workspace/rustLearn/hello", NULL );
         /* 如果exec函数返回，表明没有正常执行命令，打印错误信息*/
        printf("no come to here\n");
         exit( 0 );
      }
      else {/* 父进程， 等待子进程结束，并打印子进程的返回值 */
         printf( " child process return \n");
         sleep(10);
         exit(0);
      }
  
}