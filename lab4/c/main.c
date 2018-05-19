#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <limits.h>
#include "util.h"

int main(int argc, char *argv[]) {
  // check args
  if (argc < 2) {
    printf("Usage: %s [page frames]\n", argv[0]);
    return 1;
  }

  // create cache with size of input
  int page_frames = strtol(argv[1], 0, 10);

  if (page_frames > INT_MAX) {
    printf("Invalid page frames.\n");
    return 1;
  }

  int cache[20] = {0};
  int distances[20] = {0};
  int pos = -1;

  /**
   * Array "beginning" is at pos
   * 
   * [0, 0, 0, 0, 0, 0]
   *  ^ pos = 0
   * 
   * [4, 8, 15, 16, 0, 0]
   *            ^ pos = 3
   * 
   * pos = newest page
   * lower index = older
   */

  // handle page requests
  int input;
  while (1 == scanf("%d", &input)) {
    size_t cache_size = sizeof(cache) / sizeof(cache[0]);
    size_t distances_size = sizeof(distances) / sizeof(distances[0]);
    int index = indexOf(cache, cache_size, input);
    if (index != -1) {
      // found item, increment the distance count
      // pos - index = distance
      // Example:
      // [4, 8, 15, 16, 23, 42]
      //     ^ index = 1    ^ pos = 5
      // 5 - 1 = 4 from pos
      int dist = pos - index;
      distances[dist]++;

      // remove from array
      if (removeIndex(cache, cache_size, index)) {
        // probably not going to be reached
        fprintf(stderr, "Error while removing index: Index out of bounds\n");
        return 1;
      }

      // print out page that caused page fault
      if (dist > page_frames) {
        printf("Page fault: %d\n", input);
      }
    } else {
      // only move pos if not in array
      // since page is removed from cache if found
      pos++;
    }

    // add the input page to "beginning"
    cache[pos] = input;

    printArray(cache, cache_size);
    printArray(distances, distances_size);
  }

  return 0;
}
