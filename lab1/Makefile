CC = gcc
CFLAGS = -std=c99
SRCS = $(wildcard *.c)
PROGS = $(patsubst %.c,%,$(SRCS))

.PHONY: default clean
default: all
all: $(PROGS)

%: %.c
	$(CC) $(CFLAGS) $< -o $@.o

clean:
	-rm -f *.o
