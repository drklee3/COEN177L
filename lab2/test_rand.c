#include <stdio.h>
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

  // return 0 if not modified
  if (before == q) {
    return 0;
  }

  // print out only modified priorities
  printf("%2d => %2d\n", before, q);
  return 1;
}

int main() {
  srand(time(NULL));

  int modified = 0;
  printf("Modified priorities:\n");
  for (int i = 0; i < TRIALS; i++) {
    for (int j = 0; j < NR_SCHED_QUEUES; j++) {
      if (modify(j)) {
        modified++;
      }
    }
  }

  // number of total modification attempts
  int total = TRIALS * NR_SCHED_QUEUES;
  float percentage_modified = (modified * 1.0 / total * 1.0) * 100;
  printf("%.*f%% modified / %d total\n", 2, percentage_modified, total);

  // only those q <= 3
  int over_three = TRIALS * (NR_SCHED_QUEUES - 4);
  float over_three_percentage = (modified * 1.0 / over_three * 1.0) * 100;
  printf("%.*f%% modified / %d total (excluding q <= 3)\n", 2, over_three_percentage, over_three);
  
  return 0;
}
