#include <stdint.h>
#include <stdio.h>




typedef __uint16_t uint16_t;
typedef __int16_t int16_t;
typedef long int ptrdiff_t;

int
fn (const int16_t * const kker, const uint16_t * const s, const int iws,
    ptrdiff_t in_linesize, const int16_t * const vv, const int16_t * const uu)
{
  int tmp = 0;
  for (int j = 0; j < 3; j++)
    {
      tmp += kker[iws + j] * s[vv[iws + j] * in_linesize + uu[iws + j]];
}}
