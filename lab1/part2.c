#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int main() {
  for (int i = 0; i < 8; i++) {
    if (fork() == 0) {
      // child
      printf("parentId = %d, myId = %d\n", getppid(), getpid());
    } else {
      int status;
      waitpid(-1, &status, 0);
      break;
    }
  }
  
  return 0;
}
