#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int main() {
  int current_level = 0;
  int z = 0;

  /**
   * 2-3 child processes, 17 total
   * 
   *                     x                  main process - 3 children
   *        z            x           x      level 1 - 3 children each
   *    a   b   c      x x x       x x x    level 2 - 0-3 children
   *   xxx  xx                              level 3 - no children
   * 
   *  a = 3 children, b = 2 children
   *  i = 0,          i = 1
   */
  for (int i = 0; i < 3; i++) {
    // add a flag for the z child
    if (current_level == 0 && i == 0) {
      z = 1;
    } else if (current_level == 0 && i) {
      z = 0;
    }

    int pid = fork();

    if (pid == 0) {
      // child
      current_level += 1;
      printf("i = %d, level = %d, parentId = %d, myId = %d\n", i, current_level, getppid(), getpid());

      // only for children of node z
      if (z && current_level == 2) {
        if (i == 0) {
          i = -1; // node a - 3 children
        } else if (i == 1) {
          i = 0; // node b - 2 children
        } else {
          break; // node c - no children
        }
      } else {
        // other children
        if (current_level < 2) {
          // 3 child nodes for all previous levels
          i = -1;
        } else {
          // all other nodes on level 2
          break;
        }
      }
      
    } else {
      int status;
      waitpid(-1, &status, 0);
    }
  }

  return 0;
}
