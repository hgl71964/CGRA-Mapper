// Source is: /home/alex/.local/share/compy-Learn/1.0/ffmpeg/content/libavfilter/vf_gblur.c

#include <stdint.h>
#include <stdio.h>




typedef __uint8_t uint8_t;

int
fn (float *bptr, int x, uint8_t * dst, const int width)
{
  for (x = 0; x < width; x++)
    {
      dst[x] = bptr[x];
    }
}
