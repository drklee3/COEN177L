#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  // check args
  if (argc < 2) {
    printf("Usage: %s [file]\n", argv[0]);
    return 1;
  }

  // attempt to open file
  FILE *file = fopen(argv[1], "r");

  // check if failed to open file
  if (!file) {
    printf("Error: Failed to open file.\n");
    return -1;
  }

  // get file size
  fseek(file, 0L, SEEK_END);
  long long fileSize = ftell(file);
  fseek(file, 0L, SEEK_SET);

  printf("Attempting to read a %lld byte file sequentially.\n", fileSize);

  // read file
  int count = 0;
  while(fgetc(file) != EOF) {}

  // finished
  printf("Finished reading file.\n");

  return 0;
}
