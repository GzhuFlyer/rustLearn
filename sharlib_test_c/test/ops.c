#include<stdio.h>

#include "ops.h"

static int x=0;

int get ()

{


    return x;

}

int set (int a)
{


    x = a;

    return x;

}