#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int modify(int q) {
  int NR_SCHED_QUEUES = 16;
  printf("before q: %d\n", q);
  q = rand() % 10 == 0 && q > 3 ? q + (rand() % (NR_SCHED_QUEUES - q)) : q;
  printf("after q: %d\n", q);
  return q;
}

int main() {
  printf("rand: %d\n", rand());
  modify(1);
  modify(2);
  modify(3);
  modify(4);
  modify(5);
  modify(6);
  modify(7);
  modify(8);
  modify(9);
  return 0;
}
