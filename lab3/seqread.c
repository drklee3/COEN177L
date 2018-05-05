#include <stdio.h>
#include <stdlib.h>
#include <time.h>

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
  long fileSize = ftell(file);
  fseek(file, 0L, SEEK_SET);

  // format start time
  time_t start = time(0);
  char buff_start[100];
  strftime(buff_start, 100, "%Y-%m-%dT%H:%M:%S", localtime(&start));

  // print size / start time
  printf("Attempting to read a %d byte file sequentially at %s.\n", fileSize, buff_start);

  // read file
  int _;
  int count = 0;
  while((_ = fgetc(file)) != EOF) {}

  // get time difference
  time_t end = time(0);
  time_t diff = difftime(end, start);

  // format end time
  char buff_end[100];
  strftime(buff_end, 100, "%Y-%m-%dT%H:%M:%S", localtime(&end));
  printf("Finished reading file at %s (took %lld seconds)\n", buff_end, diff);

  return 0;
}
