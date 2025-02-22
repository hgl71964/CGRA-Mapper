#include <stdint.h>
#include <stdio.h>




typedef __uint8_t uint8_t;

int
fn (int y, int i, int x, int stride, const uint8_t ** c, int h, int w,
    int bpc, const uint8_t * src)
{
  for (i = 0; i < 49; i++)
    {
      int xoff =
	((x + ((i % 7) - 3)) >=
	 0 ? (x + ((i % 7) - 3)) : (-(x + ((i % 7) - 3))));
      int yoff =
	((y + (i / 7) - 3) >= 0 ? (y + (i / 7) - 3) : (-(y + (i / 7) - 3)));
      xoff = xoff >= w ? 2 * w - 1 - xoff : xoff;
      yoff = yoff >= h ? 2 * h - 1 - yoff : yoff;
      c[i] = src + xoff * bpc + yoff * stride;
}}
