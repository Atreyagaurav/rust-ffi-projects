#include "numrng.h"
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>

void print_numrng(char *rngstr);

int main(int argc, char *argv[]) {
  if (argc < 2) {
    char *buffer = NULL;
    long unsigned int size=0;
    ssize_t n;
    while((n=getline(&buffer, &size, stdin)) > 0) {
      if (n == 1){
	/* only '\n' */
	continue;
      }
      buffer[n] = '\0';
      print_numrng(buffer);
    }
    free(buffer);
  } else {
    int i;
    for (i = 1; i < argc; i++) {
      char *rngstr = *(argv + i);
      print_numrng(rngstr);
    }
  }
  return 0;
}

void print_numrng(char *rngstr) {
  int c;
  int issigned = 0;
  for (c = 0; *(rngstr + c) != '\0'; c++) {
    if (*(rngstr + c) == ':') {
      issigned = 1;
      break;
    }
  }

  uint32_t length;
  if (issigned) {
    int32_t *numbers;
    numbers = numrng_signed(rngstr, &length);
    uint32_t i = 0;
    for (; i < length; i++) {
      printf("%d\n", *(numbers + i));
    }
  } else {
    uint32_t *numbers;
    numbers = numrng_unsigned(rngstr, &length);
    uint32_t i = 0;
    for (; i < length; i++) {
      printf("%d\n", *(numbers + i));
    }
  }
}
