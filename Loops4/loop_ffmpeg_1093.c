// Source is: /home/alex/.local/share/compy-Learn/1.0/ffmpeg/content/libavfilter/vf_blend.c

#include <stdint.h>
#include <stdio.h>




typedef __uint16_t uint16_t;
typedef long int ptrdiff_t;

int
fn (const uint16_t * bottom, ptrdiff_t width, const float opacity,
    const uint16_t * top, uint16_t * dst)
{
  for (int j = 0; j < width; j++)
    {
      dst[j] =
	top[j] +
	(((top[j] ==
	   0) ? 0 : ((1 << 16) - 1) -
	  ((((((1 << 16) - 1) - bottom[j]) * (((1 << 16) - 1) -
					      bottom[j])) / top[j]) >
	   (((1 << 16) - 1)) ? (((1 << 16) -
				 1)) : (((((1 << 16) - 1) -
					  bottom[j]) * (((1 << 16) - 1) -
							bottom[j])) /
					top[j]))) - top[j]) * opacity;
}}
