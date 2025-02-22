#include <stdint.h>
#include <stdio.h>




typedef __uint16_t uint16_t;
typedef __uint8_t uint8_t;

int
fn (int src_stride, int i, uint16_t * dst, int dst_stride, int width,
    const uint8_t * src)
{
  for (i = 0; i < width; i += 2)
    {
      dst[(0) * dst_stride + (0) * 3 + 2] =
	dst[(0) * dst_stride + (1) * 3 + 2] =
	dst[(1) * dst_stride + (1) * 3 + 2] =
	dst[(1) * dst_stride + (0) * 3 + 2] =
	(src[(1) * src_stride + 1 * (1)]);
      dst[(0) * dst_stride + (1) * 3 + 1] = (src[(0) * src_stride + 1 * (1)]);
      dst[(0) * dst_stride + (0) * 3 + 1] =
	dst[(1) * dst_stride + (1) * 3 + 1] =
	((unsigned int) (src[(0) * src_stride + 1 * (1)]) +
	 (unsigned int) (src[(1) * src_stride + 1 * (0)])) >> 1;
      dst[(1) * dst_stride + (0) * 3 + 1] = (src[(1) * src_stride + 1 * (0)]);
      dst[(1) * dst_stride + (1) * 3 + 0] =
	dst[(0) * dst_stride + (0) * 3 + 0] =
	dst[(0) * dst_stride + (1) * 3 + 0] =
	dst[(1) * dst_stride + (0) * 3 + 0] =
	(src[(0) * src_stride + 1 * (0)]);
      src += 2 * 1;
      dst += 6;
}}
