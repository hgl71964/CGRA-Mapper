#include <stdint.h>
#include <stdio.h>




typedef __int32_t int32_t;

int
fn (int32_t * p1, int i, int32_t * p2, int length)
{
  for (i = 0; i < length; i++)
    {
      int32_t a = p1[i];
      int32_t b = p2[i];
      p1[i] = b - a;
    }
}
