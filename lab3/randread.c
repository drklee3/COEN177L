#include <sys/types.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
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
  long long fileSize = ftell(file);
  fseek(file, 0L, SEEK_SET);

  // format start time
  time_t start = time(0);
  char buff_start[100];
  strftime(buff_start, 100, "%Y-%m-%dT%H:%M:%S", localtime(&start));

  // print size / start time
  printf("Attempting to read a %lld byte file randomly at %s.\n", fileSize, buff_start);

  // seed rand
  srand(time(NULL));

  int count = 0;
  int res;
  long long rand_num;
  double perc;

  while(count < fileSize) {
    do {
      // generate random u64 number
      rand_num = rand();
      rand_num = (rand_num << 32) | rand();
      // seek to random position
      res = fseek(file, rand_num % fileSize, SEEK_SET);
      // retry if fseek failed
    } while(res);
    fgetc(file); // returns something but dont care
    ++count;

    // print percent status every 1k
    perc = (count * 1.0 / fileSize * 1.0) * 100.0;
    if (count % 1000 == 0) {
      printf("Percent complete: %.*f%%\r", 2, perc);
    }
  }

  // get time difference
  time_t end = time(0);
  time_t diff = difftime(end, start);

  // format end time
  char buff_end[100];
  strftime(buff_end, 100, "%Y-%m-%dT%H:%M:%S", localtime(&end));
  printf("Finished reading file at %s (took %ld seconds)\n", buff_end, diff);

  return 0;
}
