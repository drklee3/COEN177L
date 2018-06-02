# Final Lab Proposal

short paragraph or two describing your planned project topic and general strategy

Single vs Multi threaded vs Asynchronous performance and resource consumption

The programs we have created so far are largely single threaded programs. However with different workloads, some programs may require more resources with different requirements and limitations. Generally, spreading out work done in multiple threads allows for concurrent processing and thus allow for more work done in the same amount of time.

This topic may be used to show the benefits and costs of each of the following applications:

* Single threaded
* Multi threaded - one thread per job with no thread limitations
* Multi threaded - one thread per job in a threadpool (limited to n threads)
* Single threaded asynchronous
* Multi threaded asynchronous - threadpool for running tasks

Tested with the following workloads:

* I/O bound
* CPU bound

These tests can also be used to show the overhead of synchronization primitives.

The general strategy will be to create a TCP / HTTP server for each of the above then benchmark performance and resource consumption for different workloads. The single and multithreaded synchronous programs could be tested with simple tasks in a loop instead of a server.
