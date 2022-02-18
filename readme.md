
# ThreadLake

Thread lake is a high level homogeneous thread pool manager

It is perfectly suited to tasks than can be broken down into similar () jobs and divided amongst threads for parallel execution

This crate doesn't just spawn a few threads, it is capable of

## Why?

I found that most multithreading jobs were similar, and each time I was writing the same boiler plate code. Sharing resources with arcs, using the available parellelism to get the optimal number of threads, so I decided to create a manager type to do it for me.

## What a thread lake isn't

This is categorically NOT a typical thread pool crate. A thread lake caters for specific types of tasks, that can be divided into smaller tasks that are the same, and sharing them among threads.

It is not as general as a thread pool, but it is(?) faster.

## Features

* Spawning `n` threads in terms of the available concurrency
* Moving data from the main thread to spawned threads automatically, via Arc
* Sending messages from spawned threads to the thread lake manager
* Raising play/pause/stop flags used by the threads
* Collect return values from the threads in an iterator
* Moving data out of a lake after threads have been joined  
* Split a vector up into mutable slices

# Usage

First, we create a lake that spawns 5 threads which print a message in screen

```rust
let lake = Builder::new(5)
    .spawn(|_: ThreadUtilities<_>| {
        println!("Hello thread");
    });

lake.join();
```

and that's it! `Builder::new` creates a new lake with 5 threads, we pass a closure that is cloned and sent to each thread for execution. Finally `ThreadLake::join` joins all threads.
Most of the time instead of a fixed number of threads, the user will want to make the most of the available parallelism. This can be done with a closure

```rust
let lake = Builder::new(|ac: Option<usize>| ac.unwrap())
    .spawn(|_: ThreadUtilities<_>| {
        println!("Hello thread");
    });

lake.join();
```

Alternatively, the type `FullParallelism` can be used instead of a closure. Using the `ThreadUtilities` object, which exposes some useful properties and functions for threads, we get the index of the thread and print this too

```rust
let lake = Builder::new(|ac: Option<usize>| ac.unwrap())
    .spawn(|x: ThreadUtilities<_>| {
        println!("Hello thread number {}", x.index());
    });

lake.join();
```


Next we create a lake that sums up elements of a vector `v`

```rust
let lake: ThreadLake<_, i64> = Builder::with_data(|ac: Option<usize>| ac.unwrap(), v)
    .spawn(|x: ThreadUtilities<_>| {
        x.split_slice(x.data()).iter().sum()
    });

println!("sum: {}", lake.join_iter().map(|x| x.unwrap()).sum::<i64>());
```

First we move our vector `v` into the lake, which we declare using `Builder::with_data` allowing us to send data to the lake. Data sent via `Builder::with_data` is moved into the lake, and an `Arc` is cloned into each thread.
We then use `ThreadUtilities::split_slice` which divides a slice into subslices for each thread based on the thread index. Finally we call `ThreadLake::join_iter` which collects the results from each thread into the final sum. 
Our final example makes use of the `Disjointer` which safely splits a vector into disjoint mutable slices. Since the slices are disjoint and no two threads can be given the same slice, there are no race conditions.

```rust
let lake = Builder::with_data(|ac: Option<usize>| ac.unwrap(), Disjointer::new(v))
    .spawn(|x: ThreadUtilities<_>| {
        for element in x.data().piece(&x) {
            *element = *element * 2 + 1;
        }
    });

let v = lake.join().unwrap().take();

println!("Results: {:?}", &v[..100]);
```

First we move the vector `v` into a lake inside a wrapper `Disjointer` which is responsible for partitioning the vector in a thread-safe way. We call `Disjointer::piece` which, similar to `ThreadUtilities::split_slice`, splits the
vector up into mutable slices. Note: It takes a reference to the `ThreadUtilities` object to get the thread index, and to prevent the user splitting an array outside of the `ThreadLake` object. Finally we get the mutated vector back by calling
join to wait for the threads to finish, try and get the underlying data (which fill fail if there are other references to the data) then take the original vector out of the `Disjointer`.