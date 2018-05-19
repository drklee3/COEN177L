#ifndef UTIL_H
#define UTIL_H

#include <stdlib.h>

/**
 * Helper functions for arrays
 */

int indexOf(int cache[], size_t size, int target);
int removeIndex(int cache[], size_t size, int index);
void printArray(int cache[], size_t size);

#endif
