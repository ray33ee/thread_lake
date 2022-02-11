use std::thread::{JoinHandle, ThreadId};

///Iterates over each thread join handle, calls join, then returns the result
pub struct JoinedIterator<M> {
    pub (crate) _it: std::vec::IntoIter<JoinHandle<M>>,
}

impl<M> Iterator for JoinedIterator<M> {
    type Item = std::thread::Result<M>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self._it.next()?;

        Some(n.join())
    }
}

///Iterates over each thread, yielding the thread id and name
pub struct ThreadIterator<'a, M> {
    pub (crate) _it: std::slice::Iter<'a, JoinHandle<M>>,
}

impl<'a, M> Iterator for ThreadIterator<'a, M> {
    type Item = (ThreadId, & 'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self._it.next()?;

        Some((n.thread().id(), n.thread().name().unwrap()))
    }
}
