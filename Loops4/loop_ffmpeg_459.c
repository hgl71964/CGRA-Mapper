// Source is: /home/alex/.local/share/compy-Learn/1.0/ffmpeg/content/libavcodec/me_cmp.c

#include <stdint.h>
#include <stdio.h>




typedef long int ptrdiff_t;
typedef __uint8_t uint8_t;

int
fn (ptrdiff_t stride, uint8_t * src, int i, int temp[64])
{
  for (i = 0; i < 8; i++)
    {
      temp[8 * i + 0] = (src[stride * i + 0]) + (src[stride * i + 1]);
      temp[8 * i + 1] = (src[stride * i + 0]) - (src[stride * i + 1]);;
      temp[8 * i + 2] = (src[stride * i + 2]) + (src[stride * i + 3]);
      temp[8 * i + 3] = (src[stride * i + 2]) - (src[stride * i + 3]);;
      temp[8 * i + 4] = (src[stride * i + 4]) + (src[stride * i + 5]);
      temp[8 * i + 5] = (src[stride * i + 4]) - (src[stride * i + 5]);;
      temp[8 * i + 6] = (src[stride * i + 6]) + (src[stride * i + 7]);
      temp[8 * i + 7] = (src[stride * i + 6]) - (src[stride * i + 7]);;
      {
	int a, b;
	a = temp[8 * i + 0];
	b = temp[8 * i + 2];
	temp[8 * i + 0] = a + b;
	temp[8 * i + 2] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 1];
	b = temp[8 * i + 3];
	temp[8 * i + 1] = a + b;
	temp[8 * i + 3] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 4];
	b = temp[8 * i + 6];
	temp[8 * i + 4] = a + b;
	temp[8 * i + 6] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 5];
	b = temp[8 * i + 7];
	temp[8 * i + 5] = a + b;
	temp[8 * i + 7] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 0];
	b = temp[8 * i + 4];
	temp[8 * i + 0] = a + b;
	temp[8 * i + 4] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 1];
	b = temp[8 * i + 5];
	temp[8 * i + 1] = a + b;
	temp[8 * i + 5] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 2];
	b = temp[8 * i + 6];
	temp[8 * i + 2] = a + b;
	temp[8 * i + 6] = a - b;
      };
      {
	int a, b;
	a = temp[8 * i + 3];
	b = temp[8 * i + 7];
	temp[8 * i + 3] = a + b;
	temp[8 * i + 7] = a - b;
      };
}}
