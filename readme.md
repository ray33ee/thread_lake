
# ThreadLake

Thread lake is a high level thread pool manager

This crate doesn't just spawn a few threads, it is capable of

* Spawning `n` threads in terms of the available concurrency
* Moving data from the main thread to spawned threads automatically, via Arc
* Sending messages from spawned threads to the thread lake manager
* Raising play/pause/stop flags used by the threads