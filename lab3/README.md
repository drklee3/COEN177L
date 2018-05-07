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

2.5G
 Time:   real (mean ± σ): 15.170 ±  0.051 [user 14.637 sys 0.527]
Range:       (min … max): 15.12  … 15.24

5G
 Time:   real (mean ± σ): 30.827 ±  0.161 [user 29.733 sys 1.090]
Range:       (min … max): 30.60  … 30.96
```

### Random

```text
1G
 Time:   real (mean ± σ):  1280.710 ±   36.772 [user 214.317 sys 1066.507]
Range:       (min … max):  1235.500 … 1325.570

2.5G
 Time:   real (mean ± σ):  3280.583 ±  130.395 [user 551.370 sys 2729.470]
Range:       (min … max):  3171.080 … 3463.830

5G
 Time:   real (mean ± σ):  6390.163 ±   36.237 [user 1047.047 sys 5343.670]
Range:       (min … max):  6357.670 … 6440.730
```

* use wallclock time
* run each file 3 times for both rand/seqread
* find relations in data
* explain why it spent the amount of time
