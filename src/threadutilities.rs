
use std::sync::mpsc::{Sender, SendError};
use std::sync::{Arc, Mutex};
use std::ops::Deref;

#[derive(Clone)]
pub enum Signal {
    None,
    Play,
    Pause,
    Stop,
}

pub struct ThreadUtilities<M> {
    pub (crate) _id: usize,
    pub (crate) _max_count: usize,
    pub (crate) _name: String,
    pub (crate) _check: Arc<Mutex<Signal>>,
    pub (crate) _message: Sender<M>,
}

impl<M> ThreadUtilities<M> {

    pub fn id(&self) -> usize {
        self._id
    }

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

    pub fn send(&self, message: M) -> Result<(), SendError<M>> {
        self._message.send(message)
    }

    pub fn range(&self, total: usize) -> std::ops::Range<usize> {
        let width = total / self._max_count;

        if total % self._max_count != 0 && self._id == self._max_count - 1 {
            self._id*width..(self._id+1)*width+total % self._max_count
        } else {
            self._id*width..(self._id+1)*width
        }
    }

}