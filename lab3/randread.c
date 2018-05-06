#include <sys/types.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

const int LOADING_WIDTH = 30;

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

  printf("Attempting to read a %lld byte file randomly.\n", fileSize);

  // seed rand
  srand(time(NULL));

  long long count = 0; // has to be u64 or will overflow
  long long rand_num;
  int res;
  double perc;

  // progress bar stuff
  char s1[] = "##############################";
  char s2[] = "..............................";

  // time display
  time_t start = time(0);
  time_t now;
  char buff_eta[100];
  char buff_elapsed[100];

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

    if (count % 100000 == 0) {
      // print percent status every 1k
      perc = (count * 1.0 / fileSize * 1.0);
      float perc_100 = perc * 100.0; // percent to display

      now = time(0);
      time_t diff = now - start;
      float perc_left = 100 - perc_100;
      time_t eta = (diff / perc_100) * perc_left;
      strftime(buff_eta, 100, "%H:%M:%S", gmtime(&eta));
      strftime(buff_elapsed, 100, "%H:%M:%S", gmtime(&diff));

      printf("\rProgress: %6.*f%% [%.*s%.*s] ETA %s (%s elapsed)",
        2, perc_100,
        (int)(LOADING_WIDTH * perc), s1,
        (int)(LOADING_WIDTH * (1 - perc)), s2,
        buff_eta, buff_elapsed);
      fflush(stdout);
    }
  }

  printf("\nFinished reading file.\n");

  return 0;
}
