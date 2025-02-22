#define NPOINTS 256
#define NSTAGES 8

void kernel(float data_real[],float data_imag[], float coef_real[],
            float coef_imag[])
/* data_real:         real data points */
/* data_imag:         imaginary data points */
/* coef_real:         real coefficient points */
/* coef_imag:         imaginary coefficient points */
{
  int i;
  int j;
  int k;
  float temp_real;
  float temp_imag;
  float Wr;
  float Wi;

  int groupsPerStage = 1;
  int buttersPerGroup = NPOINTS / 2;
  for (i = 0; i < NSTAGES; ++i) {
    for (j = 0; j < groupsPerStage; ++j) {
      Wr = coef_real[(1<<i)-1+j];
      Wi = coef_imag[(1<<i)-1+j];
//#pragma clang loop vectorize(enable) vectorize_width(4) unroll_count(4)
#pragma clang loop vectorize(disable) unroll_count(1)
      for (k = 0; k < buttersPerGroup; ++k) {
        temp_real = Wr * data_real[2*j*buttersPerGroup+buttersPerGroup+k] -
                    Wi * data_imag[2*j*buttersPerGroup+buttersPerGroup+k];
        temp_imag = Wi * data_real[2*j*buttersPerGroup+buttersPerGroup+k] +
                    Wr * data_imag[2*j*buttersPerGroup+buttersPerGroup+k];
        data_real[2*j*buttersPerGroup+buttersPerGroup+k] =
                    data_real[2*j*buttersPerGroup+k] - temp_real;
        data_real[2*j*buttersPerGroup+k] += temp_real;
        data_imag[2*j*buttersPerGroup+buttersPerGroup+k] =
                    data_imag[2*j*buttersPerGroup+k] - temp_imag;
        data_imag[2*j*buttersPerGroup+k] += temp_imag;
      }
    }
    groupsPerStage <<= 1;
    buttersPerGroup >>= 1;
  }
}
