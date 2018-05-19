#include <stdio.h>
#include <stdlib.h>

int parseArguments(int argc, char *argv[]) {
	int tableSize;
	if (argc == 2 && (tableSize = atoi(argv[1]))) {
		// validate negative input
		return tableSize;
	}

	fprintf(stderr, "Wrong arguments, Pass tableSize as arg\n");
	exit(-1);
}

int isInMemory(int pageRequest, int *pageTable, int tableSize) {
	for (int i = 0; i < tableSize; ++i) {
		if (pageRequest == pageTable[i]) {
			return 1;
		}
	}

	return 0;
}


int main(int argc, char *argv[]) {
	int tableSize = parseArguments(argc, argv);
	int pageRequest;
	int pageTableIndex = 0;
	int numRequest = 0;  // total page requests
	int numMisses = 0;   // page faults
	int *pageTable = (int *) malloc(sizeof((int) *tableSize)); // must call free with malloc
	char *input = NULL;
	size_t inputAllocated = 0;
	size_t bytesRead;

	while((bytesRead = getline(&input, &inputAllocated, stdin)) != -1) {
		pageRequest = atoi(input); // returns 0 on error
		if (pageRequest == 0) {
			continue;
		}
		numRequest++;
		if (!isInMemory(pageRequest, pageTable, tableSize)) {
			printf("Page %d caused pagefault\n", pageRequest);
			numMisses++;
			if (pageTableIndex < tableSize) {
				pageTable[pageTableIndex++] = pageRequest;
			} else {
				// implement page replacement algo
				// fifo could be single line here
				fprintf(stderr, "Ran out of memory, implement page replacement algo\n");
			}
		} // else update something in pagetable
	}

	printf("Hit rate = %f\n", (numRequest - numMisses) / (double) numRequest);

	free(pageTable);
	return 0;
}
