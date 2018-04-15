#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int main() {
  int current_level = 0;
  
  /** 
   * This loop maintains 3 children per process.
   * Since each loop would decrease i, the loop is
   * restarted for all child processes that require
   * another level.
   * 
   * 
   *             x              main process
   *      x      x       x      level 1 - 3 children each
   *     xxx    xxx     xxx     level 2 - 3 children each
   */
  for (int i = 0; i < 3; i++) {
    int pid = fork();

    if (pid == 0) {
      // child
      current_level += 1;
      printf("i = %d, level = %d, parentId = %d, myId = %d\n", i, current_level, getppid(), getpid());
      // restart loops if they're on the correct levels
      if (current_level < 2) {
        i = -1;
      } else {
        // lowest level, don't make anymore children
        break;
      }
    } else {
      int status;
      waitpid(-1, &status, 0);
    }
  }

  return 0;
}
