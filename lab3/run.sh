#!/bin/bash
run_seq="true"
run_rand="true"
should_check_files="true"
skip_gen="false"
trials="3"
log_file="output.log"
file_path="/local/weka/"
file_suffix="bin"
# file sizes for head
file_sizes=( "1G" "2500M" "5G" )
# file sizes in bytes (ish)
file_sizes_bytes=( 1073000000 2620000000 5368000000 )
# pretty file sizes
file_list=( "1G" "2.5G" "5G" )

trap "exit" INT

# generate files
gen_files() {
  echo "Generating random files in $file_path"
  for index in ${!file_sizes[*]};
  do
    head -c ${file_sizes[$index]} < /dev/urandom > ${file_path}${file_list[$index]}.${file_suffix}
    echo "${file_list[$index]} file generated"
  done
}

# check random data files if they're readable and minimum size
check_files() {
  echo "Checking random files..."
  for index in ${!file_sizes[*]};
  do
    # build filepath
    full_file_path="${file_path}${file_list[$index]}.${file_suffix}"
    # check if readable
    if [[ -e $full_file_path && -r $full_file_path ]]
    then
      actual_size=$(stat -c%s $full_file_path)
      # check if file size large enough in case head failed or something
      if [ $actual_size -le ${file_sizes_bytes[index]} ]; then
        echo "Error: $full_file_path is not large enough.  ($actual_size bytes)"
        exit 2
      fi
    else
      echo "Cannot read $full_file_path, it doesn't exist or you don't own it."
      exit 2
    fi
  done
}

# run a test for n trials
# $1  seqread or randread executable name
# $2  filename to read
run_test() {
  # delete mtime file in case
  if [ -e /tmp/mtime.$$ ]; then
    rm /tmp/mtime.$$
  fi

  for i in $(seq 1 $trials); do
    # output read type and trial #
    if [ "$1" == "seqread" ]; then
      echo "[trial #$i / $trials] Reading $2 sequentially" |& tee -a $log_file
    else
      echo "[trial #$i / $trials] Reading $2 randomly" |& tee -a $log_file
    fi

    # run program, filter out percent complete status
    # output time command to /tmp/mtime.$$, not really safe but oh well in this case don't really care
    /usr/bin/time -f "real %e user %U sys %S" -a -o /tmp/mtime.$$ \
      ./$1.o "${file_path}${2}.${file_suffix}" |& tee >(grep -v "Progress:" >> $log_file)
    # print time to stdout from time command
    tail -1 /tmp/mtime.$$
    printf "%s\n\n" "-----------------------" |& tee -a $log_file
  done
  # finished running file n trials
  echo "=======================" |& tee -a $log_file
  echo "Finished $1 for $2 ($trials trials)" |& tee -a $log_file
  # print out average data and standard deviation for real time
  awk 'BEGIN {min=max=""} \
    /^real/ { \
      {et += $2; ut += $4; st += $6; count++; sum+=$2; sumsq+=$2*$2}; \
      if (min == "" || $2 < min) min = $2; \
      if (max == "" || $2 > max) max = $2; \
    } END \
    {  printf "\033[1;32m Time:   real (mean ± σ):  %.3f ± %.3f [user %.3f sys %.3f] \
    \nRange:       (min … max):  %.3f … %.3f\033[0m\n", \
    et/count, sqrt(sumsq/count - (sum/count)**2), ut/count, st/count, min, max }' /tmp/mtime.$$ |& tee -a $log_file
  printf "%s\n\n" "=======================" |& tee -a $log_file
  # delete mtime file
  rm /tmp/mtime.$$
}

# read each file for n trials
# $1  seqread or randread executable name
run_trial() {
  for fname in "${file_list[@]}"
  do
    echo " > Reading $fname"
    run_test $1 $fname
  done
}


# print usage
usage() {
  echo "Usage: $0 [-rsSch] [-l file] [-t trials] [-p path] [-s suffix]"
  echo "  -r          only run random reads (default: runs both)"
  echo "  -s          only run sequential reads (default: runs both)"
  echo "  -S          skip generation of random files"
  echo "  -c          skip check for random files for existance and size"
  echo "  -h          display help"
  echo "  -l file     specify log file for output (default: output.log)"
  echo "  -t trials   specify number of trials to run each (default: 3)"
  echo "  -p path     specify path for random data files (default: /local/weka/)"
  echo "  -o suffix   specify random data file suffix (default: .bin)"
  exit 1
}

# parse input flags
while getopts "rsScht:p:o:" flag; do
  case $flag in
    r) run_seq="false" ;;
    s) run_rand="false" ;;
    S) skip_gen="true" ;;
    c) should_check_files="false" ;;
    h) usage ;;
    t) trials=${OPTARG} ;;
    p) file_path="${OPTARG}" ;;
    o) file_suffix="${OPTARG}" ;;
    *) echo "Unexpected option ${flag}"; exit 2;;
  esac
done


# run stuff
if [ "$skip_gen" == "false" ]; then
  gen_files
else
  echo "Skipping generation of random files."
fi

# check if files exist and are correct size
if [ $should_check_files == "true" ]; then
  check_files
else
  echo "Skipping random file check."
fi

# compile files, if that wasn't obvious enough
echo "Compiling c files"
make

# run tests for # trials
echo "Running $trials trials"
if [ $run_seq == "true" ]; then
  run_trial seqread
else
  echo "Skipping sequential reads."
fi

if [ $run_rand == "true" ]; then
  run_trial randread
else
  echo "Skipping random reads."
fi
