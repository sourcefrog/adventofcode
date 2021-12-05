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

use proptest::prelude::*;

use aoclib::{point, Matrix};

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
    let matrix: Matrix<char> = Matrix::from_string_lines(&input);
    assert_eq!(matrix.width(), 93);
    assert_eq!(matrix.height(), 90);
    assert_eq!(matrix.to_string(), input);
}

#[test]
fn map() {
    let m = Matrix::from_linear_iter([1, 2, 3, 4, 5, 6], 2);
    let m2 = m.map(|v| *v % 3 == 0);
    assert_eq!(
        m2.as_linear_slice(),
        [false, false, true, false, false, true]
    );
}

#[test]
fn from_file() {
    let matrix = Matrix::from_file("testdata/dec11.txt");
    assert_eq!(matrix.width(), 93);
    assert_eq!(matrix.height(), 90);
}

#[test]
fn display_char_matrix() {
    let m = Matrix::from_linear_vec("abcd".chars().collect(), 2);
    let d = format!("{}", m);
    assert_eq!(d, "ab\ncd\n");
    assert_eq!(d, m.to_string());
}

#[test]
fn from_iter_iter() {
    let matrix: Matrix<usize> = (0..=1)
        .into_iter()
        .map(|row| (0..=1).into_iter().map(move |col| row * 10 + col))
        .collect();
    assert_eq!((matrix.width(), matrix.height()), (2, 2));
    assert_eq!(
        matrix.values().copied().collect::<Vec<usize>>(),
        [0, 1, 10, 11]
    );
    assert_eq!(matrix.as_linear_slice(), [0, 1, 10, 11]);
}

#[test]
#[should_panic]
fn from_linear_vec_wrong_len() {
    Matrix::from_linear_vec((0..4).collect(), 3);
}

#[test]
fn from_linear_iter() {
    let m = Matrix::from_linear_iter(0..100u32, 20);
    assert_eq!(m.width(), 20);
    assert_eq!(m.height(), 5);
}

proptest! {
    #[test]
    fn basic_proptest(content in prop::collection::vec(0..100u32, 4), x in 0usize..2, y in 0usize..2) {
        let w = 2;
        let m = Matrix::from_linear_vec(content.clone(), w);
        prop_assert_eq!(m.width(), 2);
        prop_assert_eq!(m.height(), 2);
        prop_assert_eq!(m[(x, y)], content[y * 2 + x]);
    }
}
