#1/bin/bash
echo "Generating random files in /local/weka/"
head -c 1G < /dev/urandom > /local/weka/1G.bin
echo "1G file generated"
head -c 2500M < /dev/urandom > /local/weka/2.5G.bin
echo "2.5G file generated"
head -c 5G < /dev/urandom > /local/weka/5G.bin
echo "5G file generated"

echo "Compiling c files"
make

echo "Running sequential tests"
for i in {1..3}
do
  echo "Trial $i"
  ./seqread.o /local/weka/1G.bin
done