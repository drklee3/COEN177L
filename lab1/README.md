# Lab 1

## Running

```bash
# compile all parts
make

# run the programs separately
./part1.o
./part2.o
./part3.o
./part4.o

# remove binaries
make clean
```


## Description

### Part 1

This simple shell creates an infinite loop which accepts user input from `stdin`, forks, then executes the input command as the child program. On invalid commands where `execve` fails, it would print out an error `Invalid command.` as the code would continue in the shell program. Breaking after printing this message also prevents the failed child process from also accepting user input. The parent process waits until the child process exits. The main parent process can be exited by entering `exit`.

### Part 2

This program creates 8 child processes with 1 child process each. Each process calls `fork` which creates a child process, waits until the child process exits, then exits with a `break` to exit the loop. This creates a chain of processes until it reaches a length of 8.

### Part 3

To create 12 total children with 2-3 child processes each, the following structure is made:

```
       x           level 0 - 3 children (main process)
  ┌────┼────┐
  x    x    x      level 1 - 3 children each
 ┌┼┐  ┌┼┐  ┌┼┐
 xxx  xxx  xxx     level 2 - no children
```

All child nodes would create 3 nodes except for the nodes on level 2.  Because a for loop is used, the incrementing value `i` would have to be reset for processes on level 1 in order for them to also create 3 nodes.  Children on level 2 would have no children by using `break` to exit the loop thus not creating anymore children.


### Part 4

Creating 17 total children with 2-3 child processes each is similar to part 3 with the following structure:

```
             x           level 0 - 3 children (main process)
     ┌───────┼────┐
     z       x    x      level 1 - 3 children each
 ┌───┼───┐  ┌┼┐  ┌┼┐
 a   b   x  xxx  xxx     level 2 - no children
┌┼┐ ┌┤
xxx xx 
```

Most of the child processes create 3 further children similar to before, but the processes labeled `a` and `b` create 3 and 2 additional children respectively. Because the process "tree" is now unbalanced, we need to keep track of the process `z` which is `level == 1 && i == 0` which can be assigned to a `z` flag. This is used in level 2 to determine which child process we are currently on, along with the loop value `i`.  Process `a` would be `z && level == 2 && i == 0`, while process `b` would be the same except with `i == 1`. 
