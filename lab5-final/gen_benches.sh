#!/bin/bash
# this script generates a series of rust macros for benchmarks
# of threads 1 to 8 each with a range of 0 to 100% lock holds

for threads in $(seq 1 8); do
  for percent in $(seq 0.0 .01 1.0); do
    percent_name="${percent/./}"
    printf "benchtest!{mutex_%dt_%sp, %d, %s}\n" $threads $percent_name $threads $percent
  done
  echo ""
done