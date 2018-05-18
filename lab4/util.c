#include "util.h"
#include <stdio.h>
#include <stdlib.h>

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