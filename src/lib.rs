extern crate core;

mod threadlake;
mod threadutilities;
mod joined_iterator;
mod builder;

#[cfg(test)]
mod tests {
    use crate::threadlake::ThreadLake;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn hello_lakes() {
        let mut lake = ThreadLake::new(|_| 4);

        lake.spawn(|x| {
            println!("Hello from thread {}", x.id());
        });

        lake.join();
    }

    #[test]
    fn sum() {

        let n = 1000005;

        //Create lake that will spawn 10 threads
        let mut lake = ThreadLake::new(|_| 10);

        //Spawn the threads. sum the first n integers
        lake.spawn(move |x| {
            let mut count = 0;

            for i in x.range(n) {
                count = count + i;
            }

            count
        });

        //Total up the sum from each thread
        let total: usize = lake.join_iter().map(|x| x.unwrap()).sum();

        assert_eq!(total, 500004500010);

    }
}
