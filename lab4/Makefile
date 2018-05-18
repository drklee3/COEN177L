CC = gcc
CFLAGS = -std=c99
SRCS = $(wildcard *.c)
PROGS = $(patsubst %.c,%,$(SRCS))

.PHONY: default clean submit
default: lru
all: $(PROGS)

%: %.c
	$(CC) $(CFLAGS) $< -o $@.o

clean:
	-rm -f *.o
	-rm -f *.tar

submit:
	-tar -cvf lab.tar --exclude=*.o --exclude=.git/ .
