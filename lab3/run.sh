#!/bin/bash
files_list=( "1G.bin" "2.5G.bin" "5G.bin" )

gen_files() {
  echo "Generating random files in /local/weka/"
  head -c 1G < /dev/urandom > /local/weka/1G.bin
  echo "1G file generated"
  head -c 2500M < /dev/urandom > /local/weka/2.5G.bin
  echo "2.5G file generated"
  head -c 5G < /dev/urandom > /local/weka/5G.bin
  echo "5G file generated"
}


run_test() {
  echo "Running sequential tests"
  for fname in "${files_list[@]}"
  do
    echo "Reading $fname"
    time ./seqread.o /local/weka/$fname
  done
}


if [ $1 != "skipgen" ]; then
  gen_files
fi

echo "Compiling c files"
make
run_test
