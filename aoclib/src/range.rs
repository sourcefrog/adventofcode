use std::cmp::{max, min};
use std::ops::Range;

pub trait RangeExt: Sized {
    fn overlap(&self, other: &Self) -> [Self; 3];
}

impl<R: Ord + Copy> RangeExt for Range<R> {
    /// Split self into the range that comes before `other`, that is contained within `other`,
    /// and that comes after `other`.
    ///
    /// Any of the resulting ranges might be empty: use [Range::is_empty].
    fn overlap(&self, other: &Self) -> [Self; 3] {
        [
            self.start..min(self.end, other.start),
            max(self.start, other.start)..min(self.end, other.end),
            max(self.start, other.end)..max(self.end, other.end),
        ]
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn overlap() {
        assert_eq!((0..10).overlap(&(10..20)), [0..10, 10..10, 20..20])
    }
}
