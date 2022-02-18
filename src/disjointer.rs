use std::cell::UnsafeCell;
use crate::threadutilities::ThreadUtilities;
use crate::split::SubSliceMut;

///Disjointer takes a vector, and partitions it into roughly equal sized disjoint mutable slices
///
/// A disjointer allows a thread lake to split mutable work in a vector between threads, without worrying about
/// the borrow checker, or wasting time on runtime checks. Since we split the vector into disjoint slices, we can work
/// on them mutably without fear of race conditions.
///
/// Splitting must be performed with a [`ThreadUtilities`] object via a thread lake, to ensure no two
/// distinct threads are given this same mutable slice, as this would create a race condition.
pub struct Disjointer<T>(UnsafeCell<Vec<T>>);

unsafe impl<T> Send for Disjointer<T> {}
unsafe impl<T> Sync for Disjointer<T> {}

impl<T> Disjointer<T>
    where T: 'static
{
    ///Wrap a vector-like object in a disjointer object
    pub fn new<I>(vector: I) -> Self
        where I: Into<Vec<T>>
    {
        Self(UnsafeCell::new(vector.into()))
    }

    ///Get a mutable subslice for the current thread that is unique and non-overlapping with other threads
    pub fn piece<D, M>(&self, utility: & ThreadUtilities<D, M>) -> SubSliceMut<T> {
        unsafe {
            let entire_slice = (*self.0.get()).as_mut_slice();
            utility.split_slice_mut(entire_slice)
        }
    }

    ///Unwrap the vector
    pub fn take(self) -> Vec<T> {
        self.0.into_inner()
    }


}
