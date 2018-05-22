# lab4

## Building

```bash
# install rust (https://www.rust-lang.org/en-US/install.html)
# requires at least v1.26
curl https://sh.rustup.rs -sSf | sh
# compile optimized build
cargo build --release
```

## Usage

```text
USAGE:
    page-replacements [FLAGS] [OPTIONS] <table_size> --algorithm <algorithm>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -a, --algorithm <algorithm>    Sets the page replacement algorithm to use
                                   [possible values: fifo, lru, second_chance, sc]
    -i <input>                     Input file for page file access numbers
    -o, --output <output>          Sets the output csv file to write results to
    -t, --to <to_table_size>       Sets the max page table size to test a range of sizes

ARGS:
    <table_size>    Sets the page table size
```

## Examples

```bash
# run lru with page table size of 10
cat accesses.txt | ./target/release/page-replacements 10 -a lru

# run multiple trials with a range of memory sizes (10 to 500)
# output file will have the algorithm name inserted, output.csv -> output.fifo.csv
./target/release/page-replacements 10 --to 500 -a fifo -o output.csv -i accesses.txt > /dev/null

# run lru show debug info (prints array / page table contents for each input)
# probably not a good idea to use -v with accesses.txt or large table sizes
# for second chance: blue = referenced, red = unreferenced
# multiple instances of the v flag can be used to increase log level:
# use -vv to display additional information
./target/release/page-replacements 10 -a lru -v
```

The graph was created with R, you can run the R script with `Rscript`.
This requires a CSV file for each page replacement algorithm with headers `table_size,[algorithm_name]`.

```bash
# install required R packages
$ R
> install.packages("ggplot2")
> install.packages("reshape2")
> q() # select workspace image with y

# run script
$ Rscript visualize.R
```

The Basics
The goal of this assignment is to gain experience with page replacement (and to a lesser extent, caching) algorithms. In this assignment your goal is to write programs that simulate page replacement algorithms. Your initial program is to accept at least one numeric command-line parameter, which it will use as the number of available page frames. 
For example:
$lru 27
or
$simulate -lru 7
should run a simulation of the LRU page replacement algorithm for a memory/cache size of 7 pages/blocks. But whence will page requests come? The answer is that your program should expect page requests to arrive on standard input (stdin, so a basic fgets(), or scanf(), call should suffice to read in the unsigned integer page numbers being requested). So assuming you have a sequence of page numbers in a text file called "accesses.txt" you should be able to run your simulator by typing:
$cat accesses.txt | lru 42

* size of memory / # of page frames accepted as cli arg
* output page number that is not in cache / page fault
* Total numbers from input file = total # of page/block requests
* Total numbers output from program = # requests that resulted in page fault
* page requests as individual #s 1 per line
* number = requested page number
* ignore trailing text or lines that don't start with a number
* terminate when reach EOF
* status outputs sent to stderr
* program for FIFO, LRU, Second Chance

The output of your program will be every page number that was not found to be in the cache. In other words, the output of your program will be a sequence of page numbers that represents all the incoming requests that resulted in a page fault. Using your program, you should be able to get two numbers from the unix command line (by counting the number of lines read from the input file, and the number of lines produced by your simulator). The first of these numbers is the total number of page/block requests your simulator program has received (you get this by counting the number of valid lines in your input file), and the second number is how many of these page requests did result in a page fault (you get this by counting the number of lines produced as output by your program - which is faithfully reproducing the page replacement algorithm's behavior).
Your programs are to accept page requests on stdin as individual numbers, one per line, where each number indicates the requested page number. Each program is to further ignore any trailing text on the input lines, or any lines that do not start with a number. Your program terminates its simulation when it encounters an end-of-file. Once again, the size of the memory being managed by your program (the number of page frames, or the size of the cache if you treat this as a caching algorithm) is to be accepted as a command-line argument to your program. Any status output (e.g., messages you wish to print for debugging/user) should be sent to stderr (standard error, in other words, it should be possible to use your program and see nothing in standard output other than the page-faults/cache-misses, by redirecting only stdout).
You are to provide a program for each of the following replacement algorithms: FIFO, LRU, and Second Chance Page Replacement.
Note that for Second Chance, when a page is first brought in to memory (i.e., into your array of page numbers in memory), it should have its referenced bit set to "FALSE." In other words, it starts in a state equivalent to having been given a second chance. Its referenced bit becomes "TRUE"  only after it is accessed due to a reference that follows the one that brought it into memory.

second chance

* first into memory reference bit = false
* reference bit = true when accessed

* graph of 3+
* which one's better

