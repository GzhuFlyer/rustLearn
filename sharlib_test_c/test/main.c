#include <stdio.h>

#include "ops.h"

int main (int argc, char** argv)

{

int a = 100;

int b = get ();

int c = set (a);

int d = get ();

printf ("a=%d,b=%d,c=%d,d=%d\n",a,b,c,d);

return 0;

}
// g++ ops.c -shared -g -DDEBUG -o libops.so
//g++ ops.c -shared -g -DDEBUG -o libops.so -fPIC
// g++ main.c -o app -Wall -g -lops
//  g++ main.c -o app -Wall -g -lops -L /home/frank/workspace/bky/rustLearn/sharlib_test_c/test
// g++ main.c -o app -Wall -g -lops -L /lib
// cp libops.so /lib/.
//  export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/frank/workspace/bky/rustLearn/sharlib_test_c/test
//https://visualgdb.com/gdbreference/commands/sharedlibrary
