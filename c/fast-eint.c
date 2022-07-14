
#include <stddef.h>
#include <stdint.h>

typedef __uint128_t uint128_t;

// HAC, 14.12 Algorithm Multiple-precision multiplication
void fe_widening_mul(uint64_t *w, uint64_t *x, uint64_t *y,
                     size_t digits_count) {
  for (size_t i = 0; i < 2 * digits_count; i++) {
    w[i] = 0;
  }
  uint64_t u;
  for (size_t i = 0; i < digits_count; i++) {
    uint64_t c = 0;
    for (size_t j = 0; j < digits_count; j++) {
      uint128_t uv = ((uint128_t)x[j]) * y[i] + w[i + j] + c;
      w[i + j] = (uint64_t)uv;
      u = *((uint64_t *)&uv + 1);
      c = u;
    }
    w[i + digits_count] = u;
  }
}

void fe_widening_mul_256_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                               size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    fe_widening_mul(w, x, y, 4);
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
    if (i != 0)
      printf(",");
  }
  printf("}");
}

int main() {
  {
    uint64_t res[8] = {0};
    uint64_t a[4] = {1};
    uint64_t b[4] = {1};
    fe_widening_mul(res, a, b, 4);

    dump_num(res, 8);
    printf("\n");
  }
  {
    uint64_t res[8] = {0};
    uint64_t a[4] = {12};
    uint64_t b[4] = {12};
    fe_widening_mul(res, a, b, 4);

    dump_num(res, 8);
    printf("\n");
  }
}

#endif
