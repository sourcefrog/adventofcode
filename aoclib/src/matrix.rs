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
use std::ops::{Index, IndexMut};

use crate::{point, Point};

#[derive(Clone, Eq, PartialEq)]
pub struct Matrix<T> {
    w: usize,
    h: usize,
    d: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    /// Return all values in row,col order.
    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.d.iter()
    }

    fn offset(&self, p: Point) -> usize {
        self.offset_xy(p.x as usize, p.y as usize)
    }

    fn offset_xy(&self, x: usize, y: usize) -> usize {
        assert!(x < self.w as usize);
        assert!(y < self.h as usize);
        (self.w as usize) * y + x
    }

    /// Return a vec of the 4 neighboring points (if in-range) and their
    /// values.
    ///
    /// p may have negative coordinates.
    pub fn neighbors4(&self, p: Point) -> Vec<(Point, &T)> {
        let mut v: Vec<(Point, &T)> = Vec::with_capacity(4);
        let h = self.h as isize;
        let w = self.w as isize;
        if p.y > 0 {
            v.push((p.up(), &self[p.up()]))
        }
        if p.y < (h - 1) {
            v.push((p.down(), &self[p.down()]))
        }
        if p.x > 0 {
            v.push((p.left(), &self[p.left()]))
        }
        if p.x < (w - 1) {
            v.push((p.right(), &self[p.right()]))
        }
        v
    }

    /// Iterate all point addresses in this matrix.
    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
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
}

impl<T: Clone> Matrix<T> {
    pub fn new(w: usize, h: usize, d: T) -> Matrix<T> {
        Matrix {
            w,
            h,
            d: vec![d; w * h],
        }
    }

    pub fn try_get(&self, p: Point) -> Option<T> {
        if p.x >= 0 && p.y >= 0 && p.x < self.w as isize && p.y < self.h as isize {
            Some(self.d[self.offset(p)].clone())
        } else {
            None
        }
    }

    /// Return a vec of all present 8-way neighbors.
    pub fn neighbor8_values(&self, p: Point) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(8);
        let w = self.w as isize;
        let h = self.h as isize;
        if p.y > 0 {
            if p.x > 0 {
                v.push(self[p.left().up()].clone())
            }
            v.push(self[p.up()].clone());
            if p.x < (w - 1) {
                v.push(self[p.right().up()].clone())
            }
        }
        if p.x > 0 {
            v.push(self[p.left()].clone())
        }
        if p.x < (w - 1) {
            v.push(self[p.right()].clone())
        }
        if p.y < (h - 1) {
            if p.x > 0 {
                v.push(self[p.left().down()].clone())
            }
            v.push(self[p.down()].clone());
            if p.x < (w - 1) {
                v.push(self[p.right().down()].clone())
            }
        }
        v
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_matrix() {
        let mut m = Matrix::new(10, 10, 7u8);
        assert_eq!(m[point(5, 5)], 7u8);
        m[point(6, 6)] = 10;
        assert_eq!(m[point(6, 6)], 10);
        assert_eq!(m[point(5, 5)], 7u8);

        assert_eq!(m.row(4).cloned().collect::<Vec<u8>>(), vec![7u8; 10]);
    }

    #[test]
    fn index_by_tuple() {
        let mut m = Matrix::new(10, 10, 7u8);
        assert_eq!(m[(5usize, 5)], 7u8);
        m[point(6, 6)] = 10;
        assert_eq!(m[(6usize, 6)], 10);
        assert_eq!(m[(5usize, 5)], 7u8);
    }

    #[test]
    fn from_string() {
        let input = std::fs::read_to_string("testdata/dec11.txt").unwrap();
        let matrix = Matrix::from_string_lines(&input);
        assert_eq!(matrix.width(), 93);
        assert_eq!(matrix.height(), 90);
    }

    #[test]
    fn from_file() {
        let matrix = Matrix::from_file("testdata/dec11.txt");
        assert_eq!(matrix.width(), 93);
        assert_eq!(matrix.height(), 90);
    }

    #[test]
    fn from_iter_iter() {
        let matrix: Matrix<usize> = (0..=1)
            .into_iter()
            .map(|row| (0..=1).into_iter().map(move |col| row * 10 + col))
            .collect();
        assert_eq!((matrix.width(), matrix.height()), (2, 2));
        assert_eq!(
            matrix.values().map(|v| *v).collect::<Vec<usize>>(),
            [0, 1, 10, 11]
        );
    }
}
