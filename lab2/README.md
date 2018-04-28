# Lab 2

## Building
```bash
# move to MINIX source 
cd /usr/src/
# copy files
mv *.c kernel/
# compile and reboot
make world
reboot
```
### Part 1

To modify the MINIX startup banner, the `announce()` function was found in `kernel/main.c` (line 278) and another `printf` statement was added.

### Part 2

To skew the priority scheduling, the `pick_proc()` function in `kernel/proc.c` (line 1314) was modified to randomly change priorities above 3.  The following line was added in the loop iterating over each queues:
```c
q = rand() % 10 == 0 && q > 3 ? q + (rand() % (NR_SCHED_QUEUES - q)) : q;
```

This changes the priority of a process, 
