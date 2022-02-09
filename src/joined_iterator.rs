use std::thread::JoinHandle;

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
