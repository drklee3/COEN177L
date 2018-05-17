# Lab 3

## Usage

You can use the `run.sh` script to generate random files, compile, and run the trials.
This will create a file `ouput.log` which contains individual trial data.

```text
# make run.sh executable
chmod +x run.sh

./run.sh [-rsSch] [-l file] [-t trials] [-p path] [-s suffix]
  -r          only run random reads (default: runs both)
  -s          only run sequential reads (default: runs both)
  -S          skip generation of random files
  -c          skip check for random files for existance and size
  -h          display help
  -l file     specify log file for output (default: output.log)
  -t trials   specify number of trials to run each (default: 3)
  -p path     specify path for random data files (default: /local/weka/)
  -o suffix   specify random data file suffix (default: .bin)
```

### Examples

```bash
# generate random files and run 3 trials for sequential and random reads for the 3 sized files
./run.sh

# run without generating random data files (assuming they are already made)
./run.sh -S

# run only sequential with only 1 trial and a custom directory for the random files
./run.sh -s -t 1 -p /tmp/coen/

# run 3 trials with custom suffix for random files /local/weka/1G.ye
# useful when there are other user files that may conflict
./run.sh -o ye
```

To run manually without the bash script:

```bash
# generate files with random data, repeat for 2.5G and 5G
head -c 1G < /dev/urandom > /local/weka/1G.bin

# compile c files
make

# run sequential reads, repeat for 2.5G and 5G
time ./seqread.o /local/weka/1G.bin

# run random reads, repeat
time ./randread.o /local/weka/1G.bin
```

## Results

Times in seconds, each file run with 3 trials.

### Sequential

```text
1G
 Time:   real (mean ± σ):  6.283 ±  0.047 [user 6.043 sys 0.233]
Range:       (min … max):  6.25  …  6.35
 Rate:              mean:  162.979 MB/s

2.5G
 Time:   real (mean ± σ): 15.170 ±  0.051 [user 14.637 sys 0.527]
Range:       (min … max): 15.12  … 15.24
 Rate:              mean: 168.754 MB/s

5G
 Time:   real (mean ± σ): 30.827 ±  0.161 [user 29.733 sys 1.090]
Range:       (min … max): 30.60  … 30.96
 Rate:              mean: 166.088 MB/s
```

### Random

```text
1G
 Time:   real (mean ± σ):  1280.710 ±   36.772 [user 214.317 sys 1066.507]
Range:       (min … max):  1235.500 … 1325.570
 Rate:              mean: 0.799 MB/s

2.5G
 Time:   real (mean ± σ):  3280.583 ±  130.395 [user 551.370 sys 2729.470]
Range:       (min … max):  3171.080 … 3463.830
 Rate:              mean: 0.780 MB/s

5G
 Time:   real (mean ± σ):  6390.163 ±   36.237 [user 1047.047 sys 5343.670]
Range:       (min … max):  6357.670 … 6440.730
 Rate:              mean: 0.812 MB/s
```

As shown above, sequential reads were much faster than random reads.
The file sizes did not make a difference in speed, as it was consistent for each file for both sequential and random reads. Sequential reads averaged around 165.94 MB/s read speed, which is around the max typical speed of a hard drive. However random reads averaged 0.797 MB/s reads, which are much slower.

The different file sizes did not contribute to a different read rate, 5G was ~5 times slower than 1G for both sequential and random reads. There was a linear relationship in between file sizes and time with consistent data rates. This was expected in that file sizes did not have an effect other than more data to read.

Since these tests were run on mechanical hard drives, there were physical differences between the two sequential and random reads. For sequential reads, all the data should be all in a "row." The drive head would not have to reposition itself to read the next byte as it would be in the next position. This would result in a much faster read rate and be at or near the maximum throughput of a mechanical hard drive.

For random reads, since we use `fseek()` to move to a random offset, the drive head would have reposition to the new spot for every byte read. In comparison to CPU work done, the hard drive doing I/O has much higher latency and low IOPs, thus takes much longer than sequential reads.

Aside from physical differences, there may also have been more latency due to the thread switching between the library function calls such as `fseek()` and `fgetc()` and the user-level process, as well as potential overhead from context switching and interrupt handling from the I/O request from the functions above. The actual portions of the time caused by hardware or operating system may be difficult to determine with only the current data, but a combination of both contributed to a large amount of time in reading the files randomly.

For the operating system, the cause of process time difference may be seen in the user and sys times in the results above, sequential reads had an average of 3.57% sys time (of sys + user time) while random reads had 83.70% (average user and sys times calculated from a single trial from each 1G, 2.5G, 5G). Random reads had a much higher percentage in system time, which meant a majority of the time was spent in system calls inside the kernel compared to sequential reads which had most of the time in user-mode code inside the user process (seqread.c/o). There were many more actions done by the random read program that could not be done in user mode to access hardware, in this case the HDD via the functions mentioned previously. This would have to be done for every byte resulting in more system traps to switch from user to kernel mode in order to execute the system instructions causing more overhead.

Sequential reads may seemingly have a system call every byte as we are calling `fgetc()` until it reaches EOF, so why is it much faster than random reads? First of all, `fgetc()` both gets a character (1 byte) from the file stream and the file pointer is moved to the next character. Since we are reading single bytes until the file ends, it should be the same number of loops as the random reads. However, there are not system calls and I/O done for every single `fgetc()` call. Looking at the implementation of `fgetc()` (glibc), it contains the following macro after acquiring an I/O lock:

```c
#define _IO_getc_unlocked(_fp) \
  (_IO_BE ((_fp)->_IO_read_ptr >= (_fp)->_IO_read_end, 0) \
     ? __uflow (_fp) : *(unsigned char *) (_fp)->_IO_read_ptr++)
```

This shows that not all `fgetc()` calls actually require I/O as it reads from a buffer and refills it with `__uflow(_fp)` when the pointer is past the buffer. This works well for sequential reads, allowing a minimal amount of system calls despite reading byte by byte in the user code by buffering I/O. This is contrasted with random reads where it may require a system call in order to seek to a different location of the file first with `fseek()` and the new location may not have the required byte buffered. There is likely also caching for `fseek()`, but due to the large file sizes at hand, it is also likely that the target positions are outside the buffers for both seeking and reading.
