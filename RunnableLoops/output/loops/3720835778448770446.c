#include <stdint.h>
#include <stdio.h>




typedef __uint32_t uint32_t;

int
fn (int stride, const uint32_t * src, uint32_t * dst)
{
  for (int j = 0; j < stride >> 2; j++)
    {
      dst[j] = (((src[j] >> 3) + (0x3F3F3F3F & dst[j])) << 3) & 0xFCFCFCFC;
}}
