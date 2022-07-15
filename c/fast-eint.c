
#include <stddef.h>
#include <stdint.h>

// #define FE_USE_COMBA
#define FE_MIN(x, y) (((x) < (y)) ? (x) : (y))
#define FE_MAX(x, y) (((x) > (y)) ? (x) : (y))

typedef __uint128_t uint128_t;

// HAC, 14.12 Algorithm Multiple-precision multiplication
void fe_widening_mul(uint64_t *w, uint64_t *x, uint64_t *y,
                     size_t digits_count) {
  (void)digits_count;
  for (size_t i = 0; i < 2 * digits_count; i++) {
    w[i] = 0;
  }
  for (size_t i = 0; i < digits_count; i++) {
    uint64_t c = 0;
    for (size_t j = 0; j < digits_count; j++) {
      uint128_t uv = ((uint128_t)x[j]) * y[i] + w[i + j] + c;
      w[i + j] = (uint64_t)uv;
      c = *((uint64_t *)&uv + 1);
    }
    w[i + digits_count] = c;
  }
}

void fe_widening_mul_comba(uint64_t *w, uint64_t *x, uint64_t *y,
                           size_t digits_count) {
  uint128_t c = 0;
  int pa = (int)digits_count * 2;

  for (int ix = 0; ix < pa; ix++) {
    /* get offsets into the two big nums */
    int ty = FE_MIN((int)digits_count - 1, ix);
    int tx = ix - ty;
    /* this is the number of times the loop will iterate, essentially
       while (tx++ < a->used && ty-- >= 0) { ... }
     */
    int iy = FE_MIN((int)digits_count - tx, ty + 1);
    int overflow = 0;
    for (int iz = 0; iz < iy; ++iz) {
      uint128_t product = (uint128_t)x[tx + iz] * (uint128_t)y[ty - iz];
      c += product;
      // overflow makes it very slow
      if (c < product) {
        overflow += 1;
      }
    }
    w[ix] = (uint64_t)c;
    c = c >> 64;
    if (overflow) {
      uint128_t t = (uint128_t)overflow;
      c += t << 64;
    }
  }
}

void fe_widening_mul_256_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                               size_t batch) {
  for (size_t i = 0; i < batch; i++) {
#ifdef FE_USE_COMBA
    fe_widening_mul_comba(w, x, y, 4);
#else
    fe_widening_mul(w, x, y, 4);
#endif
    w += 8;
    x += 4;
    y += 4;
  }
}

#ifdef FE_TEST
#include <stdio.h>

void dump_num(uint64_t *a, size_t len) {
  printf("{");
  for (int i = len - 1; i >= 0; i--) {
    printf("%ld", a[i]);
    if (i != 0) printf(",");
  }
  printf("}");
}

int main() {
  {
    uint64_t res[8] = {0};
    uint64_t a[4] = {1};
    uint64_t b[4] = {1};
    fe_widening_mul_256_batch(res, a, b, 1);

    dump_num(res, 8);
    printf("\n");
  }
  {
    uint64_t res[8] = {0};
    uint64_t a[4] = {12};
    uint64_t b[4] = {12};
    fe_widening_mul_256_batch(res, a, b, 1);

    dump_num(res, 8);
    printf("\n");
  }
}

#endif
