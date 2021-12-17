// Copyright 2021 Martin Pool

/*! A min-heap.
 *
 * This data can return the minimum of the inserted values
 * more efficiently than keeping everything sorted.
 *
 * The heap is logically a complete binary tree, with the constraint
 * that each node orders less than its children, if any. There is no
 * ordering constraint between children. By induction the root of the
 * tree is always the minimum.
 *
 * The heap is physically represented as a vec, with rows of the tree
 * packed into successive contiguous spans of the vec. Only the last row
 * can be incomplete and it must be packed to the left, so there are
 * no gaps in the vec.
 *
 * The vector can be thought of as a series of rows numbered from 0,
 * where row `r` is of length `1<<r`. Row `r` starts at position `(1<<r)-1`:
 * 0, 1, 3, 7, ... Each node `i` has, potentially two children, which
 * will be at `2*i + 1` and `2*i+2`. For any index `j` we can see there is
 * only one `i` such that either `j == 2*i+1` or `j == 2*i+2` and so
 * every index is used and every index has one parent.
 */

use std::iter::FromIterator;

#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    v: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    /// Construct a new empty heap.
    pub fn new() -> MinHeap<T> {
        MinHeap { v: Vec::new() }
    }

    /// Return a reference to the minimum value in the heap,
    /// if there is one, or None if the heap is empty.
    pub fn peek(&self) -> Option<&T> {
        self.v.get(0)
    }

    /// Remove and return the minimum value in the heap, if there is one.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Return the number of items in the heap.
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// True if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    /// Discard all elements from the heap.
    pub fn clear(&mut self) {
        self.v.clear()
    }

    /// Check that the heap invariants are true.
    ///
    /// Panics on failure.
    pub fn assert_valid(&self) {
        todo!();
    }

    /// Insert a new item into the heap.
    pub fn push(&mut self, t: T) {
        todo!();
    }
}

/// Returns the index of the first child of the node at index `i`.
fn left(parent: usize) -> usize {
    parent * 2 + 1
}

/// Returns the index of the right child of the node at index `i`.
fn right(parent: usize) -> usize {
    parent * 2 + 2
}

/// Return the parent of the node at this index.
fn parent(child: usize) -> usize {
    (child - 1) / 2
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        MinHeap::new()
    }
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<It>(i: It) -> MinHeap<T>
    where
        It: IntoIterator<Item = T>,
    {
        todo!()
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    fn from(v: Vec<T>) -> MinHeap<T> {
        // TODO: heapify
        todo!();
        MinHeap { v }
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use super::{left, parent, right, MinHeap};

    #[test]
    fn new_heap_is_empty() {
        let h: MinHeap<u32> = MinHeap::new();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert!(h.peek().is_none());
    }

    #[test]
    fn default_is_empty() {
        let h: MinHeap<u32> = Default::default();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert!(h.peek().is_none());
    }

    proptest! {
        #[test]
        fn parents_and_children(i in 0..1000usize) {
            assert_eq!(parent(left(i)), i);
            assert_eq!(parent(right(i)), i);
            // These don't hold at 0 which has no parent
            if i > 0 {
                // It's either left or right, not both.
                assert!(
                    (left(parent(i)) == i)
                    != (right(parent(i)) == i));
            }
        }
    }
}
