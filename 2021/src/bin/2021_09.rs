// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/9

use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use aoclib::{Matrix, Point};

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/09.txt").unwrap()
}

fn solve(input: &str) -> (u32, usize) {
    // The map as integer heights
    let map = Matrix::from_digit_lines(input);
    let mut sol_a = 0;
    // For each point in the map that's not height 9 or a low point, its
    // downhill neighbors.
    let mut downhill: Matrix<Vec<Point>> = Matrix::same_size(&map, Vec::new());
    // Points that we have to look at, where we haven't decided yet
    // which basin they're in, if any.
    let mut active: VecDeque<Point> = Default::default();
    // For points that we have decided which basin they're in: the low
    // point of that basin.
    let mut in_basin: Matrix<Option<Point>> = Matrix::same_size(&map, None);

    // For each basin, identified by its low point: the number of points
    // in that basin, including the low point.
    let mut basins: HashMap<Point, usize> = HashMap::new();

    for (p, &v) in map.point_values() {
        let ns = map.neighbors4(p);
        if ns.iter().all(|(_np, nv)| **nv > v) {
            // Everything is higher: it's a low point.
            sol_a += v + 1;
            in_basin[p] = Some(p);
            basins.insert(p, 1);
        } else if v < 9 {
            downhill[p] = ns
                .iter()
                .filter(|(_, nv)| **nv < v)
                .map(|(np, _)| *np)
                .collect();
            active.push_back(p);
        }
    }

    while let Some(p) = active.pop_front() {
        assert!(in_basin[p].is_none());
        assert!(!downhill[p].is_empty());
        let nbr_basins: Vec<Option<Point>> = downhill[p].iter().map(|np| in_basin[*np]).collect();
        if nbr_basins.iter().all(Option::is_some) {
            // all downhills have been done; are they all the same?
            if nbr_basins.iter().all_equal() {
                let lowpt = nbr_basins[0].unwrap();
                // println!("resolved {p} is in basin of {lowpt}");
                in_basin[p] = Some(lowpt);
                *basins.get_mut(&lowpt).unwrap() += 1;
            } else {
                // println!("resolved {p} is not in a single basin");
            }
        } else {
            // println!("try again later on {p}");
            active.push_back(p);
        }
    }

    let sol_b = basins.values().cloned().sorted().rev().take(3).product();

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (600, 987840));
    }

    #[test]
    fn example() {
        let example = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(solve(example), (15, 1134));
    }
}
