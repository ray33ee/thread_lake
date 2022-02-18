use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

///Represents a subslice created by splitting a slice for a thread
pub struct SubSlice<'a, T> {
    pub (crate) _slice: & 'a [T],
    pub (crate) _width: usize,
}

///Represents a mutable subslice created by splitting a slice for a thread
pub struct SubSliceMut<'a, T> {
    pub (crate) _slice: & 'a mut [T],
    pub (crate) _width: usize,
}

impl<'a, T> SubSlice<'a, T> {

    ///Return the width for this split. This loosely represents the length of the slice, unless the number of threads does not divide the length of the
    /// original slice, then the last slice has length width + remainder
    pub fn width(&self) -> usize {
        self._width
    }

    ///Return an iterator over the subslice
    pub fn iter(&self) -> std::slice::Iter<'a, T> {
        self._slice.iter()
    }
}

impl<'a, T, I: SliceIndex<[T]>> Index<I> for SubSlice<'a, T> {
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self._slice.index(index)
    }
}

impl<'a, T> IntoIterator for SubSlice<'a, T> {
    type Item = & 'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self._slice.into_iter()
    }
}

impl<'a, T> SubSliceMut<'a, T> {

    ///Return the width for this split. This loosely represents the length of the slice, unless the number of threads does not divide the length of the
    /// original slice, then the last slice has length width + remainder
    pub fn width(&self) -> usize {
        self._width
    }

    ///Return an iterator over the subslice
    pub fn iter(&self) -> std::slice::Iter<T> {
        self._slice.iter()
    }

    ///Return a mutable iterator over the subslice
    pub fn iter_mut(& mut self) -> std::slice::IterMut<T> {
        self._slice.iter_mut()
    }
}

impl<'a, T, I: SliceIndex<[T]>> Index<I> for SubSliceMut<'a, T> {
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self._slice.index(index)
    }
}

impl<'a, T, I: SliceIndex<[T]>> IndexMut<I> for SubSliceMut<'a, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self._slice.index_mut(index)
    }
}

impl<'a, T> IntoIterator for SubSliceMut<'a, T> {
    type Item = & 'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self._slice.into_iter()
    }
}
