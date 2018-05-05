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
# run without generating random data files with only 1 trial and a custom directory for the random files
./run.sh -S -t 1 -p /tmp/coen/

# run 3 trials for only sequential reads
./run.sh -s

# run 3 trials with custom suffix for random files /local/weka/1G.ye
# useful when there are other user files that may conflict
./run.sh -o ye
```

use wallclock time
run each file 3 times for both rand/seqread
find relations in data
explain why it spent the amount of time
