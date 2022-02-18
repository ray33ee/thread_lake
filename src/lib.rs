
///Main object used to manage thread pool
pub mod threadlake;

pub mod threadutilities;
pub mod iterators;

///Builder object to create thread pools
pub mod builder;

pub mod traits;

///Object used to mutably access a vector by multiple threads simultaneously
pub mod disjointer;

///Objects encapsulating a subslice created from the [`threadutilities::ThreadUtilities`] functions
pub mod split;


#[cfg(test)]
mod tests {
    use crate::threadlake::ThreadLake;
    use std::time::Duration;
    use crate::threadutilities::ThreadUtilities;
    use crate::builder::Builder;
    use crate::disjointer::Disjointer;
    use crate::traits::FullParallelism;
    use std::sync::Mutex;

    #[test]
    fn hello_lakes() {
        let lake = Builder::new(4)
            .spawn(|x: ThreadUtilities<_>| {
                println!("Hello from thread {}", x.index());
            });

        lake.join();
    }

    #[test]
    fn sum() {

        let n = 1000005;

        //Create lake that will spawn 10 threads
        let lake: ThreadLake<_, usize> = Builder::new(10)
            .spawn(move |x| {

                x.range(n).sum()
            });

        //Total up the sum from each thread
        let total: usize = lake.join_iter().map(|x| x.unwrap()).sum();

        assert_eq!(total, 500004500010);

    }

    #[test]
    fn index_search() {

        let mut test_vector: Vec<_> = (0..1000000).map(|_| 0).collect();

        test_vector[759246] = 100;

        let lake  = Builder::with_data(10, test_vector)
            .spawn(|x: ThreadUtilities<_>| {

                let v = x.data();

                let subslice = x.split_slice(v.as_slice());

                subslice.iter().enumerate().find_map(|(ind, val)| if *val != 0 { Some(ind + subslice.width() * x.index() ) } else {None})

            });

        println!("{:?}", lake.join_iter().find(|x| if let Some(_) = x.as_ref().unwrap() { true } else {false}));

    }

    #[test]
    fn simple_stop() {

        let lake = Builder::new(5)
            .spawn(|x|{

                if x.index() == 0 {
                    //0th thread sends a message to main thread
                    x.send(()).unwrap();
                }

                //If check is true, we must stop the thread
                while !x.check() {
                    std::thread::sleep(Duration::from_millis(100));
                }
            });

        //Main thread waits for the message
        lake.receiver().recv().unwrap();

        //When the main thread gets the message, we send the stop signal
        lake.stop();

        lake.join();


    }

    #[test]
    fn simple_messages() {

        let lake = Builder::new(FullParallelism)
            .spawn(|x| {
                x.send(x.index()).unwrap();

                std::thread::sleep(Duration::from_millis(100));
            });

        for _ in 0..lake.max_threads() {

            lake.receiver().recv().unwrap();

        }

        lake.join();

    }

    fn multithread_search<T, P>(data: Vec<T>, predicate: P) -> bool
        where
            T: Sync + 'static + Send,
            P: Fn(&T) -> bool + Sync + Send + 'static,
    {


        let lake= Builder::with_data(FullParallelism, (data, predicate))
            .spawn(|x: ThreadUtilities<_, _> | {
                {
                    let (data, pred) = x.data();
                    let subslice = x.split_slice(data.as_slice());

                    for  element in subslice {
                        if (pred)(element) {
                            //When we find the element, send a 'found' message to the main thread, then terminate this thread
                            x.send(Some(())).ok();
                            return
                        }
                    }
                }
                x.send(None).ok();
            });

        //We expect one response from each thread
        for _ in 0..lake.max_threads() {
            match lake.receiver().recv().unwrap() {
                Some(_) => {
                    //If a thread has found the result, return true. This could leave other worker threads still searching, but the main thread will continue
                    return true
                },
                None => {

                }
            }
        }

        false
        //lake.join_iter().any(|x| x.unwrap())
    }

    #[test]
    fn search_test() {

        let list: Vec<_> = (0..1000000).enumerate().map(|(i, _)| i ).collect();

        assert_eq!(multithread_search(list.clone(), |x| *x == 10000), true);
        assert_eq!(multithread_search(list.clone(), |x| *x == 1000001), false);

    }

    #[test]
    fn panic_test() {
        let lake = Builder::new(2)
            .names(|x: usize| format!("Panicable thread number {}", x))
            .spawn(|x: ThreadUtilities<_>| {
                println!("thread name: {}", x.name());
                panic!("This panic is deliberate, used to test that the user-specified thread names show in panic messages and ThreadUtilities");
            });

        //Assert that all threads and with an error
        assert!(lake.join_iter().all(|x| if let Err(_) = x { true } else { false }));
    }

    #[test]
    fn name_test() {
        let lake = Builder::new(3)
            .names(|x: usize| format!("My Thread {}", x))
            .spawn(|_: ThreadUtilities<_>| {});

        for (i, (_, str)) in lake.thread_iter().enumerate() {
            assert_eq!(str, format!("My Thread {}", i).as_str());
        }

        lake.join();
    }

    /*fn multithreaded_reduce<T, B, F>(list: Vec<T>, init: T, f: F) -> T
        where
            T: Sync + 'static + Send + Clone,
            F: Fn(T, &T) -> T + Sync + Send + 'static + Clone,
    {

        let lake = Builder::with_data(|x: Option<usize>| x.unwrap(), (list, f, init.clone()))
            .spawn(|x: ThreadUtilities<_>| {

                let (list, f, _) = x.data();

                let (slice, _) = x.split_slice(list.as_slice());

                let first = slice[0].clone();

                slice.iter().skip(1).fold(first, f)

            });

        let f = lake.data().1.clone();

        let mut init = init.clone();

        for element in lake.join_iter() {
            init = (f)(init, &element.unwrap());
        }

        init

    }*/

    #[test]
    fn disjoint_test() {
        let v = vec![0; 100000];

        let v = Disjointer::new(v);

        let lake = Builder::with_data(FullParallelism, v)
            .spawn(|x: ThreadUtilities<_> |{
                let mut subslice = x.data().piece(&x);

                let offset = subslice.width() * x.index();

                for (i, element) in subslice.iter_mut().enumerate() {
                    *element = *element + (i + offset); //i + offset gives the index of the entire array, i gives the index of the subslice
                }


            });

        let d = lake.join().unwrap().take();

        //Here we use another lake to verify the results, but the two algorithms could be combined into one
        let lake = Builder::with_data(FullParallelism, d)
            .spawn(|x: ThreadUtilities<_>| {
                let subslice = x.split_slice(x.data());

                let offset = subslice.width() * x.index();

                subslice.iter().enumerate().all(|(i, x)| *x == i + offset)
            });

        assert!(lake.join_iter().all(|x| x.unwrap() == true))


    }

    #[test]
    fn mutex_test() {
        let test_vector: Vec<_> = (0..100000).map(|x| x).collect();

        let results = Mutex::new(Vec::<i32>::new());

        let lake = Builder::with_data(FullParallelism, (test_vector, results))
            .spawn(|x: ThreadUtilities<_>| {

                let subslice = x.split_slice(&x.data().0);

                for element in subslice {

                    //If we find a power of 2, add it to the results array
                    if (*element as f64).log2() == (*element as f64).log2().floor() {
                        let mut res = x.data().1.lock().unwrap();

                        res.push(*element);
                    }
                }

            });

        let (_, results) = lake.join().unwrap();

        let mut results = results.into_inner().unwrap();

        results.sort();

        assert_eq!(results, vec![0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536]);

    }

}
