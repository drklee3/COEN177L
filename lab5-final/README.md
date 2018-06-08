# Final Lab

## Multithreaded performance and overhead

* CPU / IO bound task spread across different threads
* time to completion
* CPU / memory consumption

Utilizing the previous lab of page replacements due to it's CPU intensive algorithms and large amount of data to process.

Previously made to use the same number of threads as CPU cores (which *should* be optimal).

Tested with RwLock, Mutex, None

---

## Overhead of synchronization primitives

http://preshing.com/20111118/locks-arent-slow-lock-contention-is/