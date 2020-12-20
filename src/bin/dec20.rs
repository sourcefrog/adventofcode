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

#![allow(dead_code)]
#![allow(unused_imports)]
use adventofcode2020::*;
use itertools::Itertools;
use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Maps = BTreeMap<usize, Matrix<char>>;

pub fn main() {
    //    println!("a: {}", solve_a());
    println!("b: {}", solve_b());
}

fn solve_a() -> usize {
    solve_type_a(&load_input())
}

fn solve_type_a(maps: &Maps) -> usize {
    // find the canonical side-values for
    let mut by_side: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut by_square: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for (num, mat) in maps.iter() {
        let svs = side_values(&mat);
        by_square.insert(*num, svs.clone());
        println!("{} => {:?}", num, svs);
        for sv in svs {
            by_side.entry(sv).or_default().push(*num);
        }
    }
    //  dbg!(&by_side);
    // the corners are squares that have 2 sides that match other squares
    let corners = by_square
        .iter()
        .filter(|(_num, svs)| svs.iter().filter(|sv| by_side[*sv].len() == 1).count() == 2)
        .collect_vec();

    dbg!(&corners);
    assert_eq!(corners.len(), 4);

    corners.iter().map(|(n, _)| *n).product()
}

fn side_values(mat: &Matrix<char>) -> Vec<String> {
    let mut r = vec![String::new(); 4];
    for i in 0..10 {
        r[0].push(mat[point(0, i)]);
        r[1].push(mat[point(9, i)]);
        r[2].push(mat[point(i, 0)]);
        r[3].push(mat[point(i, 9)]);
    }
    // Canonical order is whichever sorts lower
    for i in 0..4 {
        r[i] = canonical(&r[i]);
    }
    r.sort();
    r
}

fn canonical(v: &str) -> String {
    let vv: String = v.chars().rev().collect();
    let v = v.to_string();
    min(v, vv)
}

fn load_input() -> Maps {
    load(&std::fs::read_to_string("input/dec20.txt").unwrap())
}

fn load(s: &str) -> Maps {
    let mut m = BTreeMap::new();
    for mut chunk in s.lines().chunks(12).into_iter() {
        let l: &str = chunk.next().unwrap();
        let num: usize = l
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();
        let mut mb = Matrix::from_rows();
        for ml in chunk.take(10) {
            mb.add_row(&ml.chars().collect_vec());
        }
        m.insert(num, mb.finish());
    }

    m
}

fn solve_b() -> usize {
    solve_type_b(&load_input())
}

fn solve_type_b(maps: &Maps) -> usize {
    // find the canonical side-values for
    let mut by_side: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut by_square: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for (num, mat) in maps.iter() {
        let svs = side_values(&mat);
        by_square.insert(*num, svs.clone());
        println!("{} => {:?}", num, svs);
        for sv in svs {
            by_side.entry(sv).or_default().push(*num);
        }
    }
    println!("{:#?}", &by_side);
    // the corners are squares that have 2 sides that match other squares
    let corners: BTreeSet<usize> = by_square
        .iter()
        .filter(|(_num, svs)| svs.iter().filter(|sv| by_side[*sv].len() == 1).count() == 2)
        .map(|(n, _svs)| *n)
        .collect();

    println!("corners: {:?}", &corners);
    assert_eq!(corners.len(), 4);

    // edges are squares that have only 1 side that matches nothing else
    let edges: BTreeSet<usize> = by_square
        .iter()
        .filter(|(_num, svs)| svs.iter().filter(|sv| by_side[*sv].len() == 1).count() == 1)
        .map(|(n, _svs)| *n)
        .collect();
    println!("{} edges: {:?}", edges.len(), edges);

    // all possible neighbors of each square
    let nbrs: BTreeMap<usize, Vec<usize>> = by_square
        .iter()
        .map(|(num, svs)| {
            (
                *num,
                svs.iter()
                    .cloned()
                    .flat_map(|sv| by_side[&sv].iter().cloned().filter(|bid| bid != num))
                    .collect_vec(),
            )
        })
        .collect();

    // let mut row: Vec<usize> = Vec::new();
    // let mut curr_id = *corners[0].0;

    let mut remaining: Vec<usize> = by_square.keys().cloned().collect();
    let top_corner: usize = *corners.iter().next().unwrap();

    let mut place = Matrix::new(12, 12, 0);
    for x in 0..12 {
        for y in 0..12 {
            let p = point(x, y);
            let next = if x == 0 && y == 0 {
                top_corner
            } else if y == 0 {
                intersect(&nbrs[&place[p.left()]], &remaining)
                    .iter()
                    .filter(|b| edges.contains(b))
                    .cloned()
                    .next()
                    .unwrap()
            } else if x == 0 {
                intersect(&nbrs[&place[p.up()]], &remaining)
                    .iter()
                    .filter(|b| edges.contains(b))
                    .cloned()
                    .next()
                    .unwrap()
            } else {
                intersect(
                    &intersect(&nbrs[&place[p.left()]], &nbrs[&place[p.up()]]),
                    &remaining,
                )[0]
            };
            println!("select {:?} => {}", p, next);
            remaining.retain(|b| *b != next);
            place[p] = next;
        }
    }

    0
}

fn intersect<T: Clone + Eq + PartialEq>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter()
        .filter(|x| b.iter().find(|y| *y == *x).is_some())
        .cloned()
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {}

    #[test]
    fn solution_b() {}
}
