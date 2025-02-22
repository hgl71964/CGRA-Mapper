#include <stdint.h>
#include <stdio.h>




typedef __uint32_t uint32_t;
typedef __int32_t int32_t;

int
fn (int32_t * samples_r, int i, uint32_t xordata, uint32_t magdata,
    int32_t * samples_l, uint32_t ordata, int nb_samples, uint32_t anddata)
{
  uint32_t crc = 0xffffffffu;
  for (i = 0; i < nb_samples; i++)
    {
      int32_t L = samples_l[i];
      int32_t R = samples_r[i];
      crc = crc * 9 + (L & 0xffff) * 3 + ((L >> 16) & 0xffff);
      crc = crc * 9 + (R & 0xffff) * 3 + ((R >> 16) & 0xffff);
      magdata |= (L < 0) ? ~L : L;
      magdata |= (R < 0) ? ~R : R;
      xordata |= L ^ -(L & 1);
      xordata |= R ^ -(R & 1);
      anddata &= L & R;
      ordata |= L | R;
    }
}
