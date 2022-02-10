
use std::sync::mpsc::{Sender, SendError};
use std::sync::{Arc, Mutex};
use std::ops::Deref;

#[derive(Clone)]
pub (crate) enum Signal {
    None,
    Play,
    Pause,
    Stop,
}

///An object sent to each thread that contains useful information and functions
pub struct ThreadUtilities<M, D> {
    pub (crate) _index: usize,
    pub (crate) _max_count: usize,
    pub (crate) _name: String,
    pub (crate) _check: Arc<Mutex<Signal>>,
    pub (crate) _message: Sender<M>,
    pub (crate) _arc: Arc<D>,
}

impl<M, D> ThreadUtilities<M, D> {

    ///Each thread is given an index in 0..max_threads, return the index
    pub fn index(&self) -> usize {
        self._index
    }

    /// The name of the thread
    pub fn name(&self) -> &String {
        &self._name
    }

    ///Check will block if a pause signal is detected (it will return after a play signal) and it will
    /// return true if a stop signal is detected
    pub fn check(&self) -> bool {

        let signal = {
            self._check.lock().unwrap().deref().clone()
        };

        match signal {
            Signal::None => {false}
            Signal::Play => {false}
            Signal::Pause => {todo!()}
            Signal::Stop => {true}
        }

    }

    ///Send data to the thread lake object
    pub fn send(&self, message: M) -> Result<(), SendError<M>> {
        self._message.send(message)
    }

    ///Can split a range from 0..total into roughly equal sized ranges, based on the thread index.
    ///
    /// Can be used to divide a list into disjoint sublists for processing
    pub fn range(&self, total: usize) -> std::ops::Range<usize> {
        let width = total / self._max_count;

        if total % self._max_count != 0 && self._index == self._max_count - 1 {
            self._index *width..(self._index +1)*width+total % self._max_count
        } else {
            self._index *width..(self._index +1)*width
        }
    }

    ///Similar to [`ThreadUtilities::range`], splits a slice into disjoint slices based on the thread index
    pub fn split_slice<'s, S>(&self, slice: & 's [S]) -> (& 's [S], usize) {
        (&slice[self.range(slice.len())], slice.len() / self._max_count)
    }

    ///Get the Arc data
    pub fn data(&self) -> Arc<D> {
        self._arc.clone()
    }

}