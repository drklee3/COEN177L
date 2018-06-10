#!/bin/bash
# this script generates a series of rust macros for benchmarks
# of threads 1 to 8 each with a range of 0 to 100% lock holds

THREADS_MIN="1"
THREADS_MAX="8"
PERCENT_MIN="0.0"
PERCENT_MAX="1.0"
PERCENT_STEP=".01"

for threads in $(seq $THREADS_MIN $THREADS_MAX); do
  for percent in $(seq $PERCENT_MIN $PERCENT_STEP $PERCENT_MAX); do
    percent_name="${percent/./}"
    echo "benchtest!{mutex_${threads}t_${percent_name}p, ${threads}, ${percent}}"
  done
  echo ""
done