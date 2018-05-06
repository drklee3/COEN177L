# Lab 3

## Usage

You can use the `run.sh` script to generate random files, compile, and run the trials.

```text
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

Times in seconds

### Sequential

```text
1G
 Time:   real (mean ± σ):  7.007 ± 0.054 [user  6.800 sys 0.207]
Range:       (min … max):  6.95  … 7.08

2.5G
 Time:   real (mean ± σ): 13.197 ± 0.145 [user 12.800 sys 0.387]
Range:       (min … max): 13.04  … 13.39

5G
 Time:   real (mean ± σ): 35.587 ± 0.512 [user 34.483 sys 1.090]
Range:       (min … max): 35.21  … 36.31
```

### Random



* use wallclock time
* run each file 3 times for both rand/seqread
* find relations in data
* explain why it spent the amount of time
