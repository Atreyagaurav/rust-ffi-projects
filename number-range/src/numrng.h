#ifndef LIB_NUMRNG
#define LIB_NUMRNG

#include <stdint.h>

int32_t *numrng_signed(char *rngstr, uint32_t *length);

uint32_t *numrng_unsigned(char *rngstr, uint32_t *length);

#endif
