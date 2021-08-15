// Copyright 2020 Google LLC
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
    w: isize,
    h: isize,
    d: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(w: usize, h: usize, d: T) -> Matrix<T> {
        Matrix {
            w: w as isize,
            h: h as isize,
            d: vec![d; w * h],
        }
    }

    /// Make a builder that will accumulate rows of a matrix.
    pub fn from_rows() -> FromRows<T> {
        FromRows::<T> {
            w: 0,
            d: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.w as usize
    }

    pub fn height(&self) -> usize {
        self.h as usize
    }

    /// Return all values in row,col order.
    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.d.iter()
    }

    fn offset(&self, p: Point) -> usize {
        (self.w as usize) * (p.y as usize) + (p.x as usize)
    }

    pub fn try_get(&self, p: Point) -> Option<T> {
        if p.x >= 0 && p.y >= 0 && p.x < self.w && p.y < self.h {
            Some(self.d[self.offset(p)].clone())
        } else {
            None
        }
    }

    /// Return a vec of the 4 neighboring points (if in-range) and their
    /// values.
    pub fn neighbors4(&self, p: Point) -> Vec<(Point, &T)> {
        let mut v: Vec<(Point, &T)> = Vec::with_capacity(4);
        if p.y > 0 {
            v.push((p.up(), &self[p.up()]))
        }
        if p.y < (self.h - 1) {
            v.push((p.down(), &self[p.down()]))
        }
        if p.x > 0 {
            v.push((p.left(), &self[p.left()]))
        }
        if p.x < (self.w - 1) {
            v.push((p.right(), &self[p.right()]))
        }
        v
    }

    /// Return a vec of all present 8-way neighbors.
    pub fn neighbor8_values(&self, p: Point) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(8);
        if p.y > 0 {
            if p.x > 0 {
                v.push(self[p.left().up()].clone())
            }
            v.push(self[p.up()].clone());
            if p.x < (self.w - 1) {
                v.push(self[p.right().up()].clone())
            }
        }
        if p.x > 0 {
            v.push(self[p.left()].clone())
        }
        if p.x < (self.w - 1) {
            v.push(self[p.right()].clone())
        }
        if p.y < (self.h - 1) {
            if p.x > 0 {
                v.push(self[p.left().down()].clone())
            }
            v.push(self[p.down()].clone());
            if p.x < (self.w - 1) {
                v.push(self[p.right().down()].clone())
            }
        }
        v
    }

    pub fn iter_points<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(
            (0..self.h).flat_map(move |y| (0..self.w).map(move |x| point(x as isize, y as isize))),
        )
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
        let w = lines.iter().map(|s| s.len()).min().unwrap() as isize;
        let h = lines.len() as isize;
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

impl<T: Clone> Index<Point> for Matrix<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        &self.d[self.offset(p)]
    }
}

impl<T: Clone> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        assert!(p.x >= 0);
        assert!(p.y >= 0);
        assert!(p.x < self.w, "{:?} too wide for {}", p, self.w);
        assert!(p.y < self.h);
        let off = self.offset(p);
        &mut self.d[off]
    }
}

pub struct FromRows<T> {
    w: usize,
    d: Vec<T>,
}

impl<T: Clone> FromRows<T> {
    pub fn add_row(&mut self, r: &[T]) {
        if self.d.is_empty() {
            // First row
            assert!(!r.is_empty());
            self.w = r.len();
        } else {
            assert_eq!(r.len(), self.w, "Rows must be the same length");
        }
        self.d.extend_from_slice(r);
    }

    pub fn finish(mut self) -> Matrix<T> {
        self.d.shrink_to_fit();
        assert!(self.d.len() % self.w == 0, "Matrix isn't rectangular");
        Matrix {
            w: self.w as isize,
            h: (self.d.len() / self.w) as isize,
            d: self.d,
        }
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
    }

    #[test]
    fn from_rows() {
        let mut b = Matrix::from_rows();
        b.add_row(&[1, 2, 3]);
        b.add_row(&[4, 5, 6]);
        b.add_row(&[7, 8, 9]);
        let m = b.finish();
        assert_eq!(m.width(), 3);
        assert_eq!(m.height(), 3);
        assert_eq!(m[point(0, 0)], 1);
        assert_eq!(m[point(2, 0)], 3);
        assert_eq!(m[point(2, 2)], 9);
    }

    #[test]
    fn from_string() {
        let input = std::fs::read_to_string("input/dec11.txt").unwrap();
        let matrix = Matrix::from_string_lines(&input);
        assert_eq!(matrix.width(), 93);
        assert_eq!(matrix.height(), 90);
    }

    #[test]
    fn from_file() {
        let matrix = Matrix::from_file("input/dec11.txt");
        assert_eq!(matrix.width(), 93);
        assert_eq!(matrix.height(), 90);
    }
}
