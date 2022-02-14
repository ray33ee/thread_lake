use std::thread::{JoinHandle, Builder};
use crate::threadutilities::{ThreadUtilities, Signal};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::iterators::JoinedIterator;
use crate::iterators::ThreadIterator;
use std::ops::Deref;

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
    _names: Vec<String>,
}

impl<M: Send + 'static, D: Sync + Send + 'static, R: Send + 'static> ThreadLake<D, R, M> {

    ///Create a thread lake with the number of threads as a closure that provides the available concurrency, and the data to send
    pub (crate) fn with_data(_max_threads: usize, data: D, names: Vec<String>) -> Self
    {
        let (_sender, _receiver) = channel();

        Self {
            _max_threads,
            _handles: Vec::with_capacity(_max_threads),
            _signal: Arc::new(Mutex::new(Signal::None)),
            _receiver,
            _sender,
            _data: Arc::new(data),
            _names: names,
        }
    }

    ///Spawn each thread in the pool
    pub (crate) fn spawn<F>(& mut self, f: F)
        where F: Fn(ThreadUtilities<D, M>) -> R + Send + 'static + Clone + Sync
    {
        let rcf = Arc::new(f);

        for id in 0..self._max_threads {

            let c = rcf.clone();

            let utility = ThreadUtilities {
                _index: id,
                _max_count: self._max_threads,
                _name: if self._names.is_empty() { format!("ThreadLake thread {}", id) } else { self._names[id].clone() },
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
    ///
    /// Moves the data out of the lake, if there are no other references to it
    pub fn join(self) -> Option<D> {
        let data = self.arc();
        for _ in self.join_iter() {

        }
        Arc::try_unwrap(data).ok()
    }

    ///An iterator over each thread, calling join and returning the result
    pub fn join_iter(self) -> JoinedIterator<R> {
        JoinedIterator { _it: self._handles.into_iter() }
    }

    /// An iterator over each thread id and thread name pair
    pub fn thread_iter(&self) -> ThreadIterator<R> {
        ThreadIterator { _it: self._handles.iter() }
    }

    ///Get the number of threads as supplied by the closure when the lake was created
    pub fn max_threads(&self) -> usize {
        self._max_threads
    }

    /// Get a reference to the data
    pub fn data(&self) -> & D {
        self._data.deref()
    }

    ///Get the data as an arc
    pub fn arc(&self) -> Arc<D> {
        self._data.clone()
    }

}


impl<M: Send + 'static, D: Sync + Send + 'static, R: Send + 'static> ThreadLake<D, R, M> {


    ///Get the receiver for messages sent from the threads
    pub fn receiver(&self) -> &Receiver<M> {
        &self._receiver
    }
}
