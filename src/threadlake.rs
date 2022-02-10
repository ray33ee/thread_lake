use std::thread::{available_parallelism, JoinHandle, Builder};
use crate::threadutilities::{ThreadUtilities, Signal};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::joined_iterator::JoinedIterator;
use crate::thread_count::ThreadCount;

///A high level thread pool
///
/// Thread lakes can automatically take care of sending messages from thread to thread lake object, sending objects via Arc to threads and pause, play and stop threads
pub struct ThreadLake<D, R, M = ()> {
    _max_threads: usize,
    _handles: Vec<JoinHandle<R>>,
    _signal: Arc<Mutex<Signal>>,
    _receiver: Receiver<M>,
    _sender: Sender<M>,
    _data: Arc<D>,
}


impl<M: std::marker::Send + 'static, R: std::marker::Send + 'static> ThreadLake<(), R, M> {

    ///Create a new thread lake with no Arc data
    pub fn new<F>(max_threads: F) -> Self
        where F: ThreadCount,
    {
        Self::with_data(max_threads, ())
    }

}

impl<M: std::marker::Send + 'static, D: std::marker::Sync + std::marker::Send + 'static, R: std::marker::Send + 'static> ThreadLake<D, R, M> {

    ///Create a thread lake with the number of threads as a closure that provides the available concurrency, and the data to send
    pub fn with_data<F>(max_threads: F, data: D) -> Self
        where F: ThreadCount,
    {
        let _max_threads = max_threads.get(available_parallelism().map(|x| x.get())) as usize; //max_threads(available_parallelism().map(|x| x.get()));

        let (_sender, _receiver) = channel();

        Self {
            _max_threads,
            _handles: Vec::with_capacity(_max_threads),
            _signal: Arc::new(Mutex::new(Signal::None)),
            _receiver,
            _sender,
            _data: Arc::new(data),
        }
    }

    ///Spawn each thread in the pool
    pub fn spawn<F>(& mut self, f: F)
        where F: Fn(ThreadUtilities<M, D>) -> R + std::marker::Send + 'static + Clone +  std::marker::Sync
    {
        let rcf = Arc::new(f);


        for id in 0..self._max_threads {

            let c = rcf.clone();

            let utility = ThreadUtilities {
                _index: id,
                _max_count: self._max_threads,
                _name: format!("Thread: {}", id),
                _check: self._signal.clone(),
                _message: self._sender.clone(),
                _arc: self._data.clone(),
            };

            let builder = Builder::new().name(utility._name.clone());

            self._handles.push(builder.spawn(move || c(utility)).unwrap());

        }

    }

    /// Continue execution for all threads
    ///
    /// This will only work for the threads that occasionally call [`ThreadUtilities::check`]
    pub fn play(&self) {
        *self._signal.lock().unwrap() = Signal::Play;
    }

    /// Stop execution for all threads
    ///
    /// This will only work for the threads that occasionally call [`ThreadUtilities::check`]
    pub fn stop(&self) {
        *self._signal.lock().unwrap() = Signal::Stop;
    }

    /// Pause execution for all threads
    ///
    /// This will only work for the threads that occasionally call [`ThreadUtilities::check`]
    pub fn pause(&self) {
        *self._signal.lock().unwrap() = Signal::Pause;
    }

    ///Iterates over [`JoinedIterator`] and consumes the results
    pub fn join(self) {
        for _ in self.join_iter() {

        }
    }

    ///An iterator over each thread, calling join and returning the result
    pub fn join_iter(self) -> JoinedIterator<R> {
        JoinedIterator { _it: self._handles.into_iter() }
    }

    ///Get the number of threads as supplied by the closure when the lake was created
    pub fn max_threads(&self) -> usize {
        self._max_threads
    }

    /// Get an Arc to the data
    pub fn data(&self) -> Arc<D> {
        self._data.clone()
    }

    ///Get the receiver for messages sent from the threads
    pub fn receiver(&self) -> &Receiver<M> {
        &self._receiver
    }

}