#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <assert.h>

const int NR_SCHED_QUEUES = 16;
const int TRIALS = 100;

int modify(int q) {
  int before = q;
  q = rand() % 10 == 0 && q > 3 ? q + (rand() % (NR_SCHED_QUEUES - q)) : q;

  // make sure q doesn't go over NR_SCHED_QUEUES
  assert(q < NR_SCHED_QUEUES);

  // print out only modified priorities
  if (before != q) {
    printf("%2d => %2d\n", before, q);
    return 1;
  }

  return 0;
}

int main() {
  srand(time(NULL));

  // number of total modification attempts
  int total = NR_SCHED_QUEUES * TRIALS;
  int modified = 0;

  printf("Modified priorities:\n");
  for (int i = 0; i < TRIALS; i++) {
    for (int i = 0; i < NR_SCHED_QUEUES; i++) {
      if (modify(i)) {
        modified++;
      }
    }
  }

  // percentage modified
  float percentage = (modified * 1.0 / total * 1.0) * 100;
  printf("%.*f%% modified / %d total\n", 2, percentage, total);

  // only those q <= 3
  int over_three = TRIALS * (NR_SCHED_QUEUES - 4);
  float over_three_percentage = (modified * 1.0 / over_three * 1.0) * 100;
  printf("%.*f%% modified / %d total (excluding q <= 3)\n", 2, over_three_percentage, over_three);
  
  return 0;
}
