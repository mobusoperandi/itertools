//! This module helps suppress two kinds of warnings: `deprecated` and `unstable_name_collisions`.
//! New items are created that are noop-wrappers of the original items.
//! The items' original paths are preserved.

use itertools::Itertools;

pub mod free {
    // it seems the compiler is not able to discern that this is used
    #[allow(dead_code)]
    pub fn zip<I, J>(i: I, j: J) -> core::iter::Zip<I::IntoIter, J::IntoIter>
        where I: IntoIterator,
              J: IntoIterator
    {
        #[allow(deprecated)]
        itertools::free::zip(i, j)
    }
}

pub trait Ext: Itertools {
    fn intersperse_wrap(self, element: Self::Item) -> itertools::Intersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        <Self as Itertools>::intersperse(self, element)
    }

    fn fold1_wrap<F>(self, f: F) -> Option<Self::Item>
        where F: FnMut(Self::Item, Self::Item) -> Self::Item,
              Self: Sized,
    {
        #[allow(deprecated)]
        <Self as Itertools>::fold1(self, f)
    }
}

impl<T: Itertools> Ext for T {}

