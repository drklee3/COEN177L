# Lab 3

## Usage

You can use the `run.sh` script to generate random files, compile, and run the trials.

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

To run manually without the bash script

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

Rate
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

Since these tests were run on mechanical hard drives, there were physical differences between the two sequential and random reads. For sequential reads, all the data should be all in a "row." The drive head would not have to reposition itself to read the next byte as the next byte would be in the next position. For random reads, since we use `fseek()` to a random offset, the drive head would have reposition to the new spot for every byte read. In comparason to CPU work done, the hard drive doing I/O has much higher latency and low IOPs, thus takes much longer. There may also have been more latency due to the thread switching between the library function calls such as `fseek()` and `fgetc()` and the user-level process, as well as potential overhead from context switching and interrupt handling from the I/O request from the functions above. The actual portions of the time caused by hardware or operating system may be difficult to determine with only the current data.

* use wallclock time
* run each file 3 times for both rand/seqread
* find relations in data
* explain why it spent the amount of time
