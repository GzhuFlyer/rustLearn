#include <errno.h>
#include <stdlib.h>
#include <time.h>
#include <fcntl.h>
#include <sys/types.h>
#define _GNU_SOURCE
#include <dirent.h>     /* Defines DT_* constants */
#include <unistd.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include<stdio.h>
#include<string.h>
#include<pthread.h>
#define MAXBUFSIZE 256

void main(int argc, char* argv[])
{
  
 printf( "=============************star main*************==============\n" );
   int dirPermitions = 0755;
   char *dir = "mountdir/myDir";
      DIR* fd = opendir(dir);
      if (NULL == fd){
      printf("opendir %s fail\n",dir);
         printf("creating dir\n");
         if (0 != mkdir(dir,dirPermitions)){
            printf("create dir %s  failed\n",dir);
         }else{
            printf("create dir %s successed\n",dir);
         }
      }
      printf("open dir success: fd = %d\n",fd);


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
         if (NULL != argv[0])
         execlp( current_absolute_path, NULL );
         /* 如果exec函数返回，表明没有正常执行命令，打印错误信息*/
        printf("come to start main second\n");
         exit( 0 );
      }
      else {/* 父进程， 等待子进程结束，并打印子进程的返回值 */
         printf( " child process return \n");
         sleep(20);
         exit(0);
      }
  
}








// #include <errno.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <time.h>
// #include<unistd.h> 
// #include<sys/types.h> 
// #include<sys/wait.h> 
// #include<stdio.h> 
// #include<stdlib.h> 
// #include<fcntl.h> 

// #define MAXBUFSIZE 256

// void err_quit(char *msg) 
// { 
//     perror(msg); 
//     exit(EXIT_FAILURE); 
// } 
// #define BUFSZ 256 

// void main(int argc, char* argv[])
// {
//    FILE* fp; 
//     char current_absolute_path[ MAXBUFSIZE ]; 
//     int count; 
//     char command[150]; 
//     char buf[BUFSZ]; 
//     count = readlink( "/proc/self/exe", current_absolute_path, MAXBUFSIZE ); 
//     if ( count < 0 || count >= MAXBUFSIZE ) { 
//     printf( "Failed\n" ); 
//     return( EXIT_FAILURE ); 
//     } 
 
//     current_absolute_path[ count ] = '\0'; 
//     printf( "/proc/self/exe -> [%s]\n", current_absolute_path ); 
//       /* 从终端读取要执行的命令 */
//     printf( "star main\n" );
//           printf("exec time is %ld\n",time(NULL));
//            printf("argv[]=%s\n",argv[0]);
//       if ( fork() == 0 ) {/* 子进程执行此命令 */
     
//       sprintf(command, "ps -C %s|wc -l", argv[0] );
//      if((fp = popen(command,"r")) == NULL) 
//         err_quit("popen"); 
//          if( (fgets(buf,BUFSZ,fp))!= NULL ) 
//          {
//             count = atoi(buf); 
//             if((count - 1) == 0) 
//                   // printf("%s not found\n",argv[0]);
//                   execlp( current_absolute_path, NULL ); 
//             else if(count ==2){
//                // execlp( current_absolute_path, NULL );
//                printf(".....\n");
//             }
//             else 
//                   printf("process : %s total is %d\n",argv[1],(count - 1)); 
//          } 
         
//          /* 如果exec函数返回，表明没有正常执行命令，打印错误信息*/
//         printf("no come to here\n");
//          exit( 0 );
//       }
//       else {/* 父进程， 等待子进程结束，并打印子进程的返回值 */
//          printf( " child process return \n");
//          sleep(10);
//          exit(0);
//       }
  
// }