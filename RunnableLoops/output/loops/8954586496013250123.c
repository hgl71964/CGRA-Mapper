#include <stdint.h>
#include <stdio.h>




typedef __int16_t int16_t;

int
fn (int z2, int tmp5, int z3, int z5, int tmp4, int tmp2, int tmp10, int tmp7,
    int tmp11, int tmp12, int tmp1, int tmp3, int16_t * dataptr, int tmp0,
    int z4, int z1, int tmp13, int ctr, int tmp6)
{
  for (ctr = 8 - 1; ctr >= 0; ctr--)
    {
      tmp0 = dataptr[8 * 0] + dataptr[8 * 7];
      tmp7 = dataptr[8 * 0] - dataptr[8 * 7];
      tmp1 = dataptr[8 * 1] + dataptr[8 * 6];
      tmp6 = dataptr[8 * 1] - dataptr[8 * 6];
      tmp2 = dataptr[8 * 2] + dataptr[8 * 5];
      tmp5 = dataptr[8 * 2] - dataptr[8 * 5];
      tmp3 = dataptr[8 * 3] + dataptr[8 * 4];
      tmp4 = dataptr[8 * 3] - dataptr[8 * 4];
      tmp10 = tmp0 + tmp3;
      tmp13 = tmp0 - tmp3;
      tmp11 = tmp1 + tmp2;
      tmp12 = tmp1 - tmp2;
      dataptr[8 * 0] = (((tmp10 + tmp11) + (1 << ((4) - 1))) >> (4));
      dataptr[8 * 4] = (((tmp10 - tmp11) + (1 << ((4) - 1))) >> (4));
      z1 = ((tmp12 + tmp13) * (((int32_t) 4433)));
      dataptr[8 * 2] =
	(((z1 + ((tmp13) * (((int32_t) 6270)))) +
	  (1 << ((13 + 4) - 1))) >> (13 + 4));
      dataptr[8 * 6] =
	(((z1 + ((tmp12) * (-((int32_t) 15137)))) +
	  (1 << ((13 + 4) - 1))) >> (13 + 4));
      z1 = tmp4 + tmp7;
      z2 = tmp5 + tmp6;
      z3 = tmp4 + tmp6;
      z4 = tmp5 + tmp7;
      z5 = ((z3 + z4) * (((int32_t) 9633)));
      tmp4 = ((tmp4) * (((int32_t) 2446)));
      tmp5 = ((tmp5) * (((int32_t) 16819)));
      tmp6 = ((tmp6) * (((int32_t) 25172)));
      tmp7 = ((tmp7) * (((int32_t) 12299)));
      z1 = ((z1) * (-((int32_t) 7373)));
      z2 = ((z2) * (-((int32_t) 20995)));
      z3 = ((z3) * (-((int32_t) 16069)));
      z4 = ((z4) * (-((int32_t) 3196)));
      z3 += z5;
      z4 += z5;
      dataptr[8 * 7] =
	(((tmp4 + z1 + z3) + (1 << ((13 + 4) - 1))) >> (13 + 4));
      dataptr[8 * 5] =
	(((tmp5 + z2 + z4) + (1 << ((13 + 4) - 1))) >> (13 + 4));
      dataptr[8 * 3] =
	(((tmp6 + z2 + z3) + (1 << ((13 + 4) - 1))) >> (13 + 4));
      dataptr[8 * 1] =
	(((tmp7 + z1 + z4) + (1 << ((13 + 4) - 1))) >> (13 + 4));
      dataptr++;
    }
}
