#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
  char *input = NULL;
  size_t size;

  while (1) {
    getline(&input, &size, stdin);
    input[strlen(input) - 1] = 0; // remove the newline at end?

    // exit on "exit"
    if (strcmp(input, "exit") == 0) {
      return 0;
    }

    if (fork() == 0) {
      // child code
      execve(input, argv, 0);
      printf("Invalid command.\n"); // if execve failed, print err message
      break; // exits invalid program
    } else {
      // parent
      int status;
      waitpid(-1, &status, 0);
    }
  }

  return 0;
}
