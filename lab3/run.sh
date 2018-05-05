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

# run a test for each of the files
run_test() {
  for fname in "${file_list[@]}"
  do
    (printf "Reading $fname file\n" |& tee -a $log_file)
    # run program, filter out percent complete status
    (time ./$1.o "${file_path}${fname}.${file_suffix}") |& tee >(grep -v "Percent complete:" >> $log_file)
    (printf "%s\n\n" "-----------------------" |& tee -a $log_file)
  done
}

# run a trial, either seq or random
run_trial() {
  for i in $(seq 1 $trials); do
    if [ "$1" == "seqread" ]; then
      echo "Starting Trial $i (sequential)" |& tee -a $log_file
    else
      echo "Starting Trial $i (random)" |& tee -a $log_file
    fi
    run_test $1
    echo "Finished Trial $i" |& tee -a $log_file
    (printf "%s\n\n" "=======================" |& tee -a $log_file)
  done
}

# print usage
usage() {
  echo "Usage: $0 [-rsSch] [-l file] [-t trials] [-p path] [-s suffix]"
  echo "  -r          only run random reads (default runs both)"
  echo "  -s          only run sequential reads (default runs both)"
  echo "  -S          skip generation of random files"
  echo "  -c          skip check for random files for existance and size"
  echo "  -h          display help"
  echo "  -l file     specify log file for output (default output.log)"
  echo "  -t trials   specify number of trials to run each (default 3)"
  echo "  -p path     specify path for random data files (default /local/weka/)"
  echo "  -o suffix   specify random data file suffix (default .bin)"
  exit 1
}

# parse input flags
while getopts "rsScht:p:" flag; do
  case $flag in
    r) run_seq="false" ;;
    s) run_rand="false" ;;
    S) skip_gen="true" ;;
    c) should_check_files="false" ;;
    h) usage ;;
    t) trials=${OPTARG} ;;
    p) file_path="${OPTARG}" ;;
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