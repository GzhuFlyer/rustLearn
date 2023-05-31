int ArraySum(int *array, int n)
{
 int t = 0;
 for(int i=0; i<n; ++i) 
    t += array[i];
 return t;
}
int main() 
{
 int a[5] = {1, 2, 3, 4, 5 };
 int sum = ArraySum(a, 5);
 return sum;
}

//gcc -std=c99 -S -o sum.s sum.c