#include <stdint.h>
#include <stdio.h>




typedef __int16_t int16_t;

int
fn (int16_t * tmp2, int k, int16_t len, const int16_t ilbc_state[8],
    int16_t maxVal, int16_t * tmp1)
{
  for (k = 0; k < len; k++)
    {
      (*tmp1) =
	(int16_t) ((((int32_t)
		     (((int16_t) (maxVal)) *
		      ((int16_t) (ilbc_state[(*tmp2)])))) + 262144) >> 19);
      tmp1++;
      tmp2--;
    }
}
