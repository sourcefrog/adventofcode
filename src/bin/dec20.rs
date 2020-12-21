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

// OK how about a different approach for part B:
//
// The squares can each be in any of 8 orientations. If we name the faces A, B,
// C, D (going clockwise from the north), then any of them can be oriented to be
// north, and that constrains its opposite to be south. Then we have one more
// choice of which square comes next in clockwise order, for which there are two
// options. That constraints the final space to be the opposite.
//
// This also constrains the order of bits on each of the faces: we can't
// independently flip just the face without also flipping the ordering of the
// faces.
//
// To arrange all the squares: we know how to find faces that match (module
// flips) and by elimination the faces that must be on the outside of the
// puzzle, and then from that we can find the corners.
//
// First then, we find one corner, and rotate it so that the outside faces are to the
// top and left.
//
// Then, find a block that can match the left block of this face, and rotate it so
// that the matching face is on the left and in the correct orientation.

#![allow(dead_code)]
#![allow(unused_imports)]
use adventofcode2020::*;
use itertools::Itertools;
use std::cmp::min;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Maps = BTreeMap<usize, Matrix<char>>;

const MAPTILES: isize = 12;

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
        let svs = canonical_side_values(&mat);
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

fn canonical_side_values(mat: &Matrix<char>) -> Vec<String> {
    let mut r = side_values(mat);
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
    load(&input())
}

fn input() -> String {
    std::fs::read_to_string("input/dec20.txt").unwrap()
}

type TileId = usize;

#[derive(Debug, Eq, Clone, PartialEq, Default)]
struct Orientation {
    flipx: bool,
    flipy: bool,
    flipxy: bool,
}

impl Orientation {
    fn all() -> Vec<Orientation> {
        let mut v = Vec::new();
        for &flipx in &[true, false] {
            for &flipy in &[true, false] {
                for &flipxy in &[true, false] {
                    v.push(Orientation {
                        flipx,
                        flipy,
                        flipxy,
                    })
                }
            }
        }
        v
    }
}

// Return the values for the sides of this tile,
// in order: N, E, S, W. Horizontal edges are read across, vertical edges are read down.
fn side_values(mat: &Matrix<char>) -> Vec<String> {
    let mut svs = vec![String::new(); 4];
    for i in 0..10 {
        svs[0].push(mat[point(i, 0)]);
        svs[1].push(mat[point(9, i)]);
        svs[2].push(mat[point(i, 9)]);
        svs[3].push(mat[point(0, i)]);
    }
    svs
}

fn rotate(p: Point, ori: &Orientation, size: isize) -> Point {
    let Point { mut x, mut y } = p;
    if ori.flipxy {
        x = p.y;
        y = p.x;
    }
    if ori.flipx {
        x = size - 1 - x;
    }
    if ori.flipy {
        y = size - 1 - y;
    }
    point(x, y)
}

const TILESZ: isize = 10;

fn rotated_side_values(mat: &Matrix<char>, ori: &Orientation) -> Vec<String> {
    let mut svs = vec![String::new(); 4];
    for i in 0..TILESZ {
        svs[0].push(mat[rotate(point(i, 0), ori, TILESZ)]);
        svs[1].push(mat[rotate(point(9, i), ori, TILESZ)]);
        svs[2].push(mat[rotate(point(i, 9), ori, TILESZ)]);
        svs[3].push(mat[rotate(point(0, i), ori, TILESZ)]);
    }
    svs
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
    Puzzle::new(&input()).solve_type_b()
}

struct Puzzle {
    maps: Maps,
    placement: Matrix<TileId>,
    oris: Matrix<Orientation>,
    unplaced_tiles: BTreeSet<TileId>,
    by_side: BTreeMap<String, Vec<TileId>>,
    by_square: BTreeMap<TileId, Vec<String>>,
}

impl Puzzle {
    fn new(s: &str) -> Puzzle {
        let maps = load(s);
        let unplaced_tiles: BTreeSet<usize> = maps.keys().cloned().collect();
        let mut by_side: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        let mut by_square: BTreeMap<usize, Vec<String>> = BTreeMap::new();

        for (tile, mat) in maps.iter() {
            let svs = canonical_side_values(&mat);
            by_square.insert(*tile, svs.clone());
            // println!("{} => {:?}", num, svs);
            for sv in svs {
                by_side.entry(sv).or_default().push(*tile);
            }
        }
        Puzzle {
            maps,
            by_side,
            by_square,
            unplaced_tiles,
            placement: Matrix::new(12, 12, 0),
            oris: Matrix::new(12, 12, Default::default()),
        }
    }

    /// Orient a tile constrained on the left and optionally above.
    fn orient_tile(&self, tile: TileId, left: Option<&str>, above: Option<&str>) -> Orientation {
        debug_assert!(left.is_some() || above.is_some());
        for ori in Orientation::all() {
            let rotsv = rotated_side_values(&self.maps[&tile], &ori);
            // println!("til {} ori {:?} rotsv {:?}", tile, ori, rotsv);
            if left.map_or(true, |a| a == rotsv[3]) && above.map_or(true, |a| a == rotsv[0]) {
                println!("found rotation {:?} for tile {}", ori, tile);
                return ori;
            }
        }
        unreachable!();
    }

    fn place_tile(&mut self, p: Point, tile: TileId, ori: Orientation) {
        let removed = self.unplaced_tiles.remove(&tile);
        assert!(removed);
        assert_eq!(self.placement[p], 0);
        self.oris[p] = ori;
        self.placement[p] = tile;
    }

    /// Return the 4 edges (N, E, S, W) of an already-placed tile.
    fn edge_values(&self, p: Point) -> Vec<String> {
        let tile = self.placement[p];
        assert_ne!(tile, 0, "no tile at {:?}", p);
        rotated_side_values(&self.maps[&tile], &self.oris[p])
    }

    fn subpixel(&self, tilept: Point, subpt: Point) -> char {
        let tile = self.placement[tilept];
        let rotpt = rotate(subpt, &self.oris[tilept], TILESZ);
        self.maps[&tile][rotpt]
    }

    fn find_corners(&self) -> Vec<TileId> {
        // the corners are squares that have 2 sides that match other squares
        let corners = self
            .by_square
            .iter()
            .filter(|(_num, svs)| svs.iter().filter(|sv| self.by_side[*sv].len() == 1).count() == 2)
            .map(|(n, _svs)| *n)
            .collect_vec();
        println!("corners: {:?}", &corners);
        assert_eq!(corners.len(), 4);
        corners
    }

    /// Find an unused tile that has the specified edge, regardless of orientation.
    fn matching_tile(&self, sv: &str) -> TileId {
        let hits = self.by_side[&canonical(sv)]
            .iter()
            .filter(|t| self.unplaced_tiles.contains(*t))
            .collect_vec();
        assert_eq!(hits.len(), 1);
        let tile = hits[0];
        println!("found match {} for {}", tile, sv);
        *tile
    }

    fn solve_type_b(&mut self) -> usize {
        println!("{:#?}", &self.by_side);

        let top_corner: usize = self.find_corners()[0];
        dbg!(top_corner);

        // Place the top corner, so that the outer edges are outside.
        let corner_outer_edges = self.by_square[&top_corner]
            .iter()
            .filter(|sv| self.by_side[*sv].len() == 1)
            .cloned()
            .collect_vec();
        assert_eq!(corner_outer_edges.len(), 2);
        dbg!(&corner_outer_edges);

        let ori = self.orient_tile(
            top_corner,
            Some(&corner_outer_edges[0]),
            Some(&corner_outer_edges[1]),
        );
        self.place_tile(point(0, 0), top_corner, ori);

        for x in 1..12 {
            let p = point(x, 0);
            let left = &self.edge_values(p.left())[1];
            // println!("look for match to {}", left);
            let tile = self.matching_tile(left);
            // println!("{} should match {}", tile, left);
            let ori = self.orient_tile(tile, Some(left), None);
            self.place_tile(p, tile, ori);
            debug_assert_eq!(left, &self.edge_values(p)[3]);
        }
        for y in 1..MAPTILES {
            for x in 0..MAPTILES {
                let p = point(x, y);
                let above = &self.edge_values(p.up())[2];
                let tile = self.matching_tile(above);
                let ori = self.orient_tile(tile, None, Some(above));
                println!("at {:?} found {} orientation {:?}", p, tile, ori);
                self.place_tile(p, tile, ori);
                if x > 0 {
                    debug_assert_eq!(self.edge_values(p.left())[1], self.edge_values(p)[3]);
                }
                debug_assert_eq!(above, &self.edge_values(p)[0]);
            }
        }

        0
    }

    fn image(&self) -> Matrix<char> {
        let sidelen = MAPTILES * (TILESZ - 1) + 1;
        let mut _image = Matrix::new(sidelen as usize, sidelen as usize, '.');

        todo!();
    }
}

fn contains<T: Clone + Eq>(a: &[T], x: &T) -> bool {
    a.iter().position(|y| *y == *x).is_some()
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
