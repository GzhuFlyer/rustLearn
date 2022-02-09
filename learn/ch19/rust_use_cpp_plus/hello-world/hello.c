#include <stdint.h>
#include <stdio.h>
 
void greet(char* s, int32_t a, int32_t b){
    printf("Hello %s! %d + %d = %d\n", s, a, b, a + b);
}