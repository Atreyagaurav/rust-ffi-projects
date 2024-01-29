#include "strtp.h"
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>


int main(int argc, char *argv[]) {
  if (argc < 2) {
    fprintf(stderr, "Provide the Template.\n");
    return 1;
  }
  char *text;
  text = strtp_render_template(argv[1], "name=gaurav\nage=22\n");
  if (text != NULL){
    printf("%s\n", text);
    free(text);
  }else{
    fprintf(stderr, "Template Couldn't be rendered.\n");
    return 1;
  }
  return 0;
}
