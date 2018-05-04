# Lab 3

```bash
# generate files with random data
head -c 1G < /dev/urandom > /local/weka/1G.bin
head -c 2.5g < /dev/urandom > /local/weka/2.5G.bin
head -c 5G < /dev/urandom > /local/weka/5G.bin
```

use wallclock time 
run each file 3 times for both rand/seqread
find relations in data
explain why it spent the amount of time
