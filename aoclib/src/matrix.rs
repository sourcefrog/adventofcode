// Copyright 2020 Google LLC
// Copyright 2021 Martin Pool
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A rectangular 2d matrix.
//!
//! Matrices are indexed by (row, column) coordinates.

use std::borrow::Borrow;
use std::cmp::max;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use crate::{point, Point};

#[derive(Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    w: usize,
    h: usize,
    d: Vec<T>,
}

impl<T> Matrix<T> {
    /// Construct by folding a 1d vec of elements
    pub fn from_linear_vec(d: Vec<T>, w: usize) -> Matrix<T> {
        assert!(w > 0);
        assert_eq!(
            d.len() % w,
            0,
            "vector length is not a multiple of the width"
        );
        let h = d.len() / w;
        Matrix { d, w, h }
    }

    /// From an iterator of elements.
    pub fn from_linear_iter<I>(i: I, w: usize) -> Matrix<T>
    where
        I: IntoIterator<Item = T>,
    {
        let d: Vec<T> = i.into_iter().collect();
        Matrix::from_linear_vec(d, w)
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    /// Return all values in row-major order.
    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.d.iter()
    }

    /// View as a linear slice in row-major order (first row first).
    pub fn as_linear_slice(&self) -> &[T] {
        &self.d
    }

    fn offset(&self, p: Point) -> usize {
        self.offset_xy(p.x as usize, p.y as usize)
    }

    fn offset_xy(&self, x: usize, y: usize) -> usize {
        assert!(x < self.w as usize);
        assert!(y < self.h as usize);
        (self.w as usize) * y + x
    }

    fn offset_to_point(&self, offset: usize) -> Point {
        debug_assert!(offset <= self.d.len());
        point((offset % self.w) as isize, (offset / self.w) as isize)
    }

    /// Iterate all point addresses in this matrix.
    pub fn points(&self) -> impl Iterator<Item = Point> {
        let h: isize = self.h as isize;
        let w: isize = self.w as isize;
        (0..h).flat_map(move |y| (0..w).map(move |x| point(x, y)))
    }

    /// Iterate all points and their values.
    pub fn point_values(&self) -> impl Iterator<Item = (Point, &T)> {
        (0..self.h).flat_map(move |y| {
            (0..self.w).map(move |x| (point(x as isize, y as isize), &self[(x, y)]))
        })
    }

    /// Iterate all the cells in a given row
    pub fn row(&self, y: usize) -> impl Iterator<Item = &T> {
        assert!(y < self.h);
        let off1 = self.w * y;
        let off2 = self.w * (y + 1);
        self.d[off1..off2].iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.h).map(move |y| self.row(y))
    }

    /// Iterate all the cells in a given column.
    pub fn column(&self, x: usize) -> impl Iterator<Item = &T> {
        assert!(x < self.w);
        (0..self.h).map(move |y| &self[(x, y)])
    }

    /// Iterate all the columns and their contents.
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.w).map(move |x| self.column(x))
    }

    /// Produce a new matrix of equal size by applying a mapping function.
    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        F: FnMut(&T) -> U,
    {
        Matrix::<U> {
            d: self.d.iter().map(f).collect(),
            w: self.w,
            h: self.h,
        }
    }

    /// Apply a cell-at-a-time update to this matrix.
    pub fn update<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for val in &mut self.d {
            f(val)
        }
    }

    pub fn contains_point(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.w as isize && p.y < self.h as isize
    }

    /// Return a vec of the 4 neighboring points (if in-range) and their
    /// values.
    ///
    /// p may have negative coordinates.
    pub fn neighbors4(&self, p: Point) -> Vec<(Point, &T)> {
        let mut v: Vec<(Point, &T)> = Vec::with_capacity(4);
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let q = p.delta(dx, dy);
            if self.contains_point(q) {
                v.push((q, &self[q]));
            }
        }
        v
    }

    /// Return a vec of all present 8-way neighbors.
    pub fn neighbor8_points(&self, p: Point) -> Vec<Point> {
        let mut v: Vec<Point> = Vec::with_capacity(8);
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let q = p.delta(dx, dy);
                if p != q && self.contains_point(q) {
                    v.push(q);
                }
            }
        }
        v
    }

    /// Return a vec of all present 8-way neighbors.
    pub fn neighbors8(&self, p: Point) -> Vec<(Point, &T)> {
        self.neighbor8_points(p)
            .into_iter()
            .map(|p| (p, &self[p]))
            .collect()
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(w: usize, h: usize, d: T) -> Matrix<T> {
        Matrix {
            w,
            h,
            d: vec![d; w * h],
        }
    }

    /// Make a new matrix the same size as `m` with a default value `fill`.
    pub fn same_size<Q>(m: &Matrix<Q>, fill: T) -> Matrix<T> {
        Matrix::new(m.width(), m.height(), fill)
    }

    /// Make a matrix that fits the bounding box of the given points, with a default value `fill`.
    pub fn bounding_box<It, BP>(it: It, fill: T) -> Matrix<T>
    where
        It: IntoIterator<Item = BP>,
        BP: Borrow<Point>,
    {
        let (w, h) = it.into_iter().fold((0, 0), |acc, p| {
            (max(acc.0, p.borrow().x), max(acc.1, p.borrow().y))
        });
        Matrix::new(w as usize + 1, h as usize + 1, fill)
    }

    pub fn try_get(&self, p: Point) -> Option<T> {
        if p.x >= 0 && p.y >= 0 && p.x < self.w as isize && p.y < self.h as isize {
            Some(self.d[self.offset(p)].clone())
        } else {
            None
        }
    }
}

impl<T> Matrix<T>
where
    T: PartialEq,
{
    /// Find the first point in the matrix where `pred` is true.
    pub fn find<Pred>(&self, pred: Pred) -> Option<Point>
    where
        Pred: FnMut(&T) -> bool,
    {
        self.d
            .iter()
            .position(pred)
            .map(|offset| self.offset_to_point(offset))
    }
}

impl Matrix<char> {
    /// Build a matrix of chars from a file containing a rectangle.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Self {
        Matrix::from_string_lines(&std::fs::read_to_string(path).unwrap())
    }

    /// Build a matrix from a string containing multiple lines.
    ///
    /// All non-empty lines must be the same length.
    pub fn from_string_lines(s: &str) -> Matrix<char> {
        let lines: Vec<&str> = s.lines().filter(|l| !l.is_empty()).collect();
        let w = lines.iter().map(|s| s.len()).min().unwrap();
        let h = lines.len();
        let d: Vec<char> = lines.iter().map(|s| s.chars()).flatten().collect();
        Matrix { w, h, d }
    }

    pub fn to_string_lines(&self) -> String {
        let mut s = String::with_capacity(self.height() * (self.width() + 1));
        let mut x = 0;
        for c in self.d.iter() {
            s.push(*c);
            x += 1;
            if x == self.w {
                s.push('\n');
                x = 0;
            }
        }
        s
    }
}

impl Matrix<bool> {
    /// Build from an iter of points: included points are true.
    pub fn from_points<It, BP>(it: It) -> Self
    where
        It: IntoIterator<Item = BP>,
        BP: Borrow<Point>,
    {
        // Make a copy so that we can calculate the bounding box and then come back to insert the
        // values...
        let ps: Vec<Point> = it.into_iter().map(|bp| *bp.borrow()).collect();
        let mut m = Matrix::bounding_box(&ps, false);
        for p in ps {
            m[p] = true;
        }
        m
    }
}

impl Matrix<u32> {
    /// Build a matrix from a string containing a rectangular matrix of characters
    pub fn from_digit_lines(s: &str) -> Matrix<u32> {
        s.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).expect("decimal digit")))
            .collect()
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: ToString + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::any::TypeId;
        use std::fmt::Write;
        let mstrings = self.map(ToString::to_string);
        let max_len: usize;
        if TypeId::of::<T>() == TypeId::of::<char>() {
            max_len = 1;
        } else {
            max_len = mstrings.values().map(|s| s.len()).max().unwrap_or(0) + 1;
        }
        for r in mstrings.rows() {
            for c in r {
                write!(f, "{:1$}", c, max_len)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.d[self.offset_xy(x, y)]
    }
}

impl<T> Index<(isize, isize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (x, y): (isize, isize)) -> &T {
        assert!(x >= 0);
        assert!(y >= 0);
        &self.d[self.offset_xy(x as usize, y as usize)]
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.d[self.offset(p)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        let o: usize = self.offset_xy(x, y);
        &mut self.d[o]
    }
}

impl<T> IndexMut<(isize, isize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut T {
        assert!(x >= 0);
        assert!(y >= 0);
        self.index_mut((x as usize, y as usize))
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        assert!(p.x >= 0);
        assert!(p.y >= 0);
        assert!(p.x < self.w as isize, "{:?} too wide for {}", p, self.w);
        assert!(p.y < self.h as isize);
        let off = self.offset(p);
        &mut self.d[off]
    }
}

/// Build a Matrix from an iterator of rows, each item of which is an iterator of cells.
///
/// All the rows must be the same length.
impl<CellIter, Cell> FromIterator<CellIter> for Matrix<Cell>
where
    CellIter: IntoIterator<Item = Cell>,
{
    fn from_iter<RowIter>(row_iter: RowIter) -> Self
    where
        RowIter: IntoIterator<Item = CellIter>,
        CellIter: IntoIterator<Item = Cell>,
    {
        let mut m = Matrix {
            w: 0,
            h: 0,
            d: Vec::new(),
        };
        for cell_iter in row_iter {
            let mut row = Vec::from_iter(cell_iter);
            if m.d.is_empty() {
                m.w = row.len();
            } else {
                assert_eq!(row.len(), m.w);
            }
            m.h += 1;
            m.d.append(&mut row);
        }
        m
    }
}
