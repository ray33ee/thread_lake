use std::thread::{available_parallelism, JoinHandle, Builder};
use crate::threadutilities::{ThreadUtilities, Signal};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::joined_iterator::JoinedIterator;

pub struct ThreadLake<M> {
    _max_threads: usize,
    _handles: Vec<JoinHandle<M>>,
    _signal: Arc<Mutex<Signal>>,
    _receiver: Receiver<M>,
    _sender: Sender<M>,
}

impl<M: std::marker::Send + 'static> ThreadLake<M> {

    pub fn new<T>(max_threads: T) -> Self
        where T: FnOnce(std::io::Result<usize>) -> usize,
    {
        let _max_threads = max_threads(available_parallelism().map(|x| x.get()));

        let (_sender, _receiver) = channel();

        Self {
            _max_threads,
            _handles: Vec::with_capacity(_max_threads),
            _signal: Arc::new(Mutex::new(Signal::None)),
            _receiver,
            _sender,
        }
    }

    pub fn spawn<F>(& mut self, f: F)
        where F: Fn(ThreadUtilities<M>) -> M + std::marker::Send + 'static + Clone +  std::marker::Sync
    {
        let rcf = Arc::new(f);


        for id in 0..self._max_threads {

            let c = rcf.clone();

            let utility = ThreadUtilities {
                _id: id,
                _max_count: self._max_threads,
                _name: format!("Thread: {}", id),
                _check: self._signal.clone(),
                _message: self._sender.clone(),
            };

            let builder = Builder::new().name(utility._name.clone());

            self._handles.push(builder.spawn(move || c(utility)).unwrap());

        }

    }

    pub fn play(&self) {
        *self._signal.lock().unwrap() = Signal::Play;
    }

    pub fn stop(&self) {
        *self._signal.lock().unwrap() = Signal::Stop;
    }

    pub fn pause(&self) {
        *self._signal.lock().unwrap() = Signal::Pause;
    }

    pub fn join(self) {
        for _ in self.join_iter() {

        }
    }

    pub fn join_iter(self) -> JoinedIterator<M> {
        JoinedIterator { _it: self._handles.into_iter() }
    }

    pub fn max_threads(&self) -> usize {
        self._max_threads
    }

}