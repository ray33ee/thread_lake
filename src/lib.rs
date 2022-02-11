
///Main object used to manage thread pool
pub mod threadlake;

pub mod threadutilities;
pub mod iterators;

///Builder object to create thread pools
pub mod builder;
mod traits;


#[cfg(test)]
mod tests {
    use crate::threadlake::ThreadLake;
    use std::time::Duration;
    use std::ops::Deref;
    use crate::threadutilities::ThreadUtilities;
    use crate::builder::Builder;

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

                let (slice, width) = x.split_slice(v.as_slice());

                slice.iter().enumerate().find_map(|(ind, val)| if *val != 0 { Some(ind + width * x.index() ) } else {None})

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

        let lake = Builder::new(|x: Option<usize>| x.unwrap())
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


        let lake= Builder::with_data(|x: Option<usize>| x.unwrap(), (data, predicate))
            .spawn(|x: ThreadUtilities<_> | {

                let d = x.data();
                let (data, pred) = d.deref();
                let (slice, _) = x.split_slice(data.as_slice());

                for element in slice {
                    if (pred)(element) {
                        return true
                    }
                }

                false
            });


        lake.join_iter().any(|x| x.unwrap())
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

}
