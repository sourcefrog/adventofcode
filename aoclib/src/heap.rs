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
 *
 * This overlaps a lot with std::collections::BinaryHeap but:
 *
 * 1. It's a min-heap which is more useful here.
 * 2. It's fun to implement.
 * 3. It's a great example to try proptest.
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

    /// Contruct a heap with a hint to the expected capacity.
    pub fn with_capacity(capacity: usize) -> MinHeap<T> {
        MinHeap {
            v: Vec::with_capacity(capacity),
        }
    }

    /// Return a reference to the minimum value in the heap,
    /// if there is one, or None if the heap is empty.
    pub fn peek(&self) -> Option<&T> {
        self.v.get(0)
    }

    /// Remove and return the minimum value in the heap, if there is one.
    pub fn pop(&mut self) -> Option<T> {
        if self.v.is_empty() {
            None
        } else {
            // To remove the 0th element: swap it to the end and return it. But then
            // the 0th node is probably too big, so bubble it down to a location
            // where it's >= both its children.
            let taken = self.v.swap_remove(0);
            let mut i = 0;
            loop {
                let il = left(i);
                let ir = right(i);
                if ir < self.v.len() {
                    // If it has two children, we must swap with the smaller of them
                    // so that the new v[i] is smaller than both its children.
                    if self.v[i] > self.v[ir] && self.v[ir] < self.v[il] {
                        self.v.swap(i, ir);
                        i = ir;
                    } else if self.v[i] > self.v[il] {
                        self.v.swap(i, il);
                        i = il;
                    } else {
                        break;
                    }
                } else if il < self.v.len() && self.v[il] < self.v[i] {
                    self.v.swap(i, il);
                    i = il;
                } else {
                    break;
                }
            }
            Some(taken)
        }
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

    /// Insert a new item into the heap.
    pub fn push(&mut self, t: T) {
        let mut i = self.v.len();
        self.v.push(t);
        while i > 0 {
            let ip = parent(i);
            if self.v[i] < self.v[ip] {
                self.v.swap(i, ip);
                i = ip;
            } else {
                break;
            }
        }
    }
}

impl<T> MinHeap<T>
where
    T: Ord + std::fmt::Debug,
{
    /// Check that the heap invariants are true.
    ///
    /// Panics on failure.
    pub fn assert_valid(&self) {
        for i in 1..self.v.len() {
            let ip = parent(i);
            debug_assert!(
                self.v[i] >= self.v[ip],
                "v[{}]={:?}, v[{}]={:?}",
                i,
                self.v[i],
                ip,
                self.v[ip]
            );
        }
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
    fn from_iter<It>(into_iter: It) -> MinHeap<T>
    where
        It: IntoIterator<Item = T>,
    {
        let iter = into_iter.into_iter();
        let mut heap: MinHeap<T> = MinHeap::with_capacity(iter.size_hint().0);
        for v in iter {
            heap.push(v);
        }
        heap
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    fn from(mut v: Vec<T>) -> MinHeap<T> {
        for i in 1..v.len() {
            let mut j = i;
            while j > 0 {
                let jp = parent(j);
                if v[j] < v[jp] {
                    v.swap(j, jp);
                    j = jp;
                } else {
                    break;
                }
            }
        }
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
        h.assert_valid();
    }

    #[test]
    fn default_is_empty() {
        let h: MinHeap<u32> = Default::default();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert!(h.peek().is_none());
        h.assert_valid();
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

        #[test]
        fn push_and_pop_one(v: isize) {
            let mut heap = MinHeap::new();
            assert!(heap.is_empty());
            assert_eq!(heap.peek(), None);
            heap.push(v);
            assert_eq!(heap.peek(), Some(&v));
            assert!(!heap.is_empty());
            assert_eq!(heap.len(), 1);

            assert_eq!(heap.pop(), Some(v));
            assert!(heap.is_empty());
            assert_eq!(heap.len(), 0);
            assert_eq!(heap.peek(), None);
            assert_eq!(heap.pop(), None);
        }

        #[test]
        fn from_vec_then_pop_everything(vals: Vec<isize>) {
            let mut heap: MinHeap<isize> = vals.clone().into();
            assert_eq!(heap.len(), vals.len());
            assert_eq!(heap.peek(), vals.iter().min());
            heap.assert_valid();
            let mut sorted = vals;
            sorted.sort_unstable();
            for v in sorted {
                assert_eq!(heap.peek(), Some(&v));
                assert_eq!(heap.pop(), Some(v));
            }
            assert!(heap.is_empty());
            assert!(heap.pop().is_none());
        }

        #[test]
        fn mixed_push_and_pop(ops: Vec<Option<isize>>) {
            let mut heap = MinHeap::new();
            // Keep a reference model as a simple sorted vec.
            let mut model = Vec::new();
            for op in ops {
                match op {
                    Some(v) => {
                        heap.push(v);
                        model.push(v);
                        model.sort_unstable_by(|a,b| b.cmp(a));
                    },
                    None => {
                        assert_eq!(heap.len(), model.len());
                        if !heap.is_empty() {
                            assert_eq!(heap.pop().unwrap(), model.pop().unwrap());
                        }
                    }
                }
            }
        }

        #[test]
        fn push(vals: Vec<isize>) {
            let mut heap: MinHeap<isize> = MinHeap::new();
            for v in &vals {
                heap.push(*v);
            }
            assert_eq!(heap.len(), vals.len());
            assert_eq!(heap.peek(), vals.iter().min());
        }

        #[test]
        fn from_vec_string_iter(mut vals: Vec<String>) {
            let mut heap: MinHeap<String> = vals.iter().cloned().collect();
            heap.assert_valid();
            assert_eq!(heap.len(), vals.len());
            assert_eq!(heap.peek(), vals.iter().min());
            vals.sort_unstable();
            for v in vals {
                assert_eq!(heap.pop().unwrap(), v);
            }
            assert!(heap.is_empty());
        }
    }
}
