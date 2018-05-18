CC = gcc
CFLAGS = -std=c99
HEADERS = util.h util.c
SRCS = $(wildcard *.c)
PROGS = $(patsubst %.c,%,$(SRCS))

.PHONY: default clean submit
default: main
all: $(PROGS)

%: %.c $(HEADERS)
	$(CC) $(CFLAGS) $< $(HEADERS) -o $@.o

clean:
	-rm -f *.o
	-rm -f *.tar

submit:
	-tar -cvf lab.tar --exclude=*.o --exclude=.git/ .
