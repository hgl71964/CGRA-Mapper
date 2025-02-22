// Source is: /home/alex/.local/share/compy-Learn/1.0/ffmpeg/content/libavfilter/af_adenorm.c

#include <stdint.h>
#include <stdio.h>




typedef __int64_t int64_t;

int
fn (const float *src, const float dc, int nb_samples, float *dst,
    const int64_t N)
{
  for (int n = 0; n < nb_samples; n++)
    {
      dst[n] = src[n] + dc * (((N + n) & 1) ? -1.f : 1.f);
}}
