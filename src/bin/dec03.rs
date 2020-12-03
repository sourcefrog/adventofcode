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

//! Solution to https://adventofcode.com/2020/day/3.

pub fn main() {
    println!("03a: {}", solve_a());
    println!("03b: {}", solve_b());
}

type Map = Vec<Vec<bool>>;

fn solve_a() -> usize {
    glide(&load_input(), 3, 1)
}

fn solve_b() -> usize {
    let map = load_input();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| glide(&map, *dx, *dy))
        .product()
}

fn tree_at(map: &Map, x: usize, y: usize) -> bool {
    map[y][x % map[y].len()]
}

fn glide(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut hits = 0;
    while y < map.len() {
        hits += tree_at(map, x, y) as usize;
        y += dy;
        x += dx;
    }
    hits
}

fn load_input() -> Map {
    std::fs::read_to_string("input/dec03.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 209);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1574890240);
    }
}
