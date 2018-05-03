# Lab 2

## Building

```bash
# copy modified files to minux dir
cp *.c /usr/src/kernel/
cd /usr/src/
# compile and reboot
make world
reboot
```

### Part 1

To modify the MINIX startup banner, the `announce()` function was found in `kernel/main.c` (line 278) and an additional `printf` statement was added.

### Part 2

The original MINIX kernel scheduling algorithm consists of a round robin scheduler with multiple queues for different priorities. Processes are added to the queues with `enqueue()` or `enqueue_head()` with a set priority. To determine which process is run, they are selected with `pick_proc()` which iterates through each queue starting with the highest priority queue.

To skew the priority scheduling, the `pick_proc()` function in `kernel/proc.c` (line 1314) was modified to randomly change priorities above 3.  The following line was added in the loop iterating over each queues:

```c
q = rand() % 10 == 0 && q > 3 ? q + (rand() % (NR_SCHED_QUEUES - q)) : q;

// or in expanded form
if (rand() % 10 == 0 && q > 3) {
    q = q + (rand() % (NR_SCHED_QUEUES - q));
}
```

To test the additional line separately:

```bash
make test_rand
./test_rand.o
```

The change made affects the priority of random processes. The first part determines whether or not it should modify the process priority, having a 10% chance of affecting only processes with a priority greater than 3 (actual percentage of modified priorities may be a bit lower shown in `test_rand.c`). The modification is just increasing q or decreasing the priority by a random amount, limiting the max value `q` can be to `NR_SCHED_QUEUES` as to not go over the number of queues. This effectively slows down the MINIX kernel as processes would have a lower priority than they should have.

In order to test if this change worked, the time it took to reboot was used. With an unmodified MINIX kernel, it took around 5 seconds to start, while the modified version took around a minute. Initially, the additional code was tested separately to see if `q` was correctly modified, as shown in `test_rand.c`. This helped determine if the modifications were correct, only affecting `q` when it is greater than 3 and only adding an amount that results in a value less than the total number of queues. However, the percent chance of the modification as well as amount modified was found by recompiling the kernel and rebooting. Ideally, testing the code would be best by having multiple runs of each modified and unmodified kernel and averaging the results, but since the modified kernel took a significantly longer time we can assume it had produced the desired result.
