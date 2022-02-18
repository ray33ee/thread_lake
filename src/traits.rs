
use std::io::Result;

///ThreadCount describes types that take the available concurrency (in the form of Option<usize) and calculate from this the number of threads to spawn
pub trait ThreadCount {
    fn get(self, available_concurrency: Result<usize>) -> usize;
}

impl ThreadCount for usize {
    fn get(self, _: Result<usize>) -> usize {
        self as usize
    }
}

impl<F> ThreadCount for F
    where F: FnOnce(Option<usize>) -> usize
{
    fn get(self, available_concurrency: Result<usize>) -> usize {
        (self)(available_concurrency.ok()) as usize
    }
}

///Used solely as a [`ThreadCount`] that makes use of the full available parallelism
pub struct FullParallelism;

impl ThreadCount for FullParallelism {
    fn get(self, available_concurrency: Result<usize>) -> usize {
        available_concurrency.unwrap()
    }
}

///Used solely as a [`ThreadCount`] that makes use of the full available parallelism minus one (for the main thread)
pub struct PartialParallelism;

impl ThreadCount for PartialParallelism {
    fn get(self, available_concurrency: Result<usize>) -> usize {
        available_concurrency.unwrap() - 1
    }
}

///ThreadName describes types that can name threads
pub trait ThreadName {
    fn get(self, index: usize) -> String;
}

impl ThreadName for String {
    fn get(self, _: usize) -> String {
        self
    }
}

impl<F> ThreadName for F
    where F: FnOnce(usize) -> String
{
    fn get(self, index: usize) -> String {
        (self)(index)
    }
}
