use std::ops::Range;

pub trait RangeExt: Sized {
    fn overlap(&self, other: &Self) -> [Self; 3];
}

impl<R> RangeExt for Range<R> {
    /// Split self into the range that comes before `other`, that is contained within `other`,
    /// and that comes after `other`.
    ///
    /// Any of the resulting ranges might be empty: use [Range::is_empty].
    fn overlap(&self, other: &Self) -> [Self; 3] {
        todo!()
    }
}
