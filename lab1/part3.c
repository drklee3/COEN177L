#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int main() {
  for (int i = 0; i < 4; i++) {
    // first layer
    if (fork() == 0) {
      printf("#%d parentId = %d, myId = %d\n", i, getppid(), getpid());
      
    } else {
      int status;
      waitpid(-1, &status, 0);
    }
  }

  return 0;
}