
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
