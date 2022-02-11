use crate::traits::ThreadCount;
use std::thread::available_parallelism;
use crate::traits::ThreadName;
use crate::threadlake::ThreadLake;
use crate::threadutilities::ThreadUtilities;
use std::time::Duration;

///Build a thread lake object
pub struct Builder<D> {

    _thread_count: usize,
    _strings: Vec<String>,
    _data: D,
    _check_sleep: Duration, //Time, in ms, to sleep between calls to check for unpause

}

impl Builder<()> {
    ///Create a builder for a thread pool with no data
    pub fn new<F>(thread_count: F) -> Self
        where F: ThreadCount
    {
        Self::with_data(thread_count, ())
    }
}

impl<D: Sync + Send + 'static> Builder<D> {
    ///Create a builder for a thread pool
    pub fn with_data<F>(thread_count: F, data: D) -> Self
        where F: ThreadCount
    {
        let _thread_count = thread_count.get(available_parallelism().map(|x| x.get()));

        Self {
            _thread_count,
            _strings: Vec::with_capacity(_thread_count),
            _data: data,
            _check_sleep: Duration::from_millis(10),
        }
    }

    ///Sets the names for each thread
    pub fn names<F>(mut self, names: F) -> Self
        where F: ThreadName + Copy
    {
        for i in 0..self._thread_count {
            self._strings.push(names.get(i));
        }
        self
    }

    ///Sets the names for each thread
    pub fn check_sleep<F>(mut self, duration: Duration) -> Self {
        self._check_sleep = duration;
        self
    }

    ///Consume the builder, spawn the threads, and return the thread lake object
    pub fn spawn<R, M, F>(self, f: F) -> ThreadLake<D, R, M>
        where M: Send + 'static, R: Send + 'static, F: Fn(ThreadUtilities<D, M>) -> R + Send + 'static + Clone +  Sync
    {

        let me = if self._strings.is_empty() {
            self.names(|x: usize| format!("ThreadLake thread {}", x))
        } else {
            self
        };

        let mut lake = ThreadLake::with_data(me._thread_count, me._data, me._strings);

        lake.spawn(f);

        lake
    }



}