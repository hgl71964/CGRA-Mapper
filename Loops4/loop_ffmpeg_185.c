// Source is: /home/alex/.local/share/compy-Learn/1.0/ffmpeg/content/libavfilter/af_afade.c

#include <stdint.h>
#include <stdio.h>




typedef __uint8_t uint8_t;

int
fn (int i, uint8_t * const *cf1, uint8_t ** dst, double gain0,
    uint8_t * const *cf0, double gain1, int c, int channels)
{
  for (c = 0; c < channels; c++)
    {
      float *d = (float *) dst[c];
      const float *s0 = (float *) cf0[c];
      const float *s1 = (float *) cf1[c];
      d[i] = s0[i] * gain0 + s1[i] * gain1;
}}
