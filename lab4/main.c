#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <limits.h>

/**
 * Gets the index of an item in array
 * 
 * @param cache array to search in
 * @param target item to search for
 * @return int Index of item, -1 if not found.
 */
int indexOf(int cache[], size_t size, int target) {
  for (int i = 0; i < size; ++i) {
    if (cache[i] == target) {
      return i;
    }
  }

  return -1;
}

/**
 * Removes an item from an array
 * 
 * @param cache array to remove from
 * @param index index of array item to remove
 * @return int 0 on success, 1 if index out of bounds
 */
int removeIndex(int cache[], size_t size, int index) {
  if (index > size) {
    return 1;
  }

  for (int i = index; i < size - 1; ++i) {
    // shift all items forward
    cache[i] = cache[i + 1];
  }

  return 0;
}

void printArray(int cache[], size_t size) {
  printf("size: %ld [", size);
  for(int i = 0; i < size; i++) {
    printf("%d, ", cache[i]);
  }
  printf("]\n");
}

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
    // if not in array output page number that results in page fault
  }

  return 0;
}
