// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/9

use aoclib::{Matrix, Point};
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/09.txt").unwrap()
    // "2199943210
    // 3987894921
    // 9856789892
    // 8767896789
    // 9899965678"
    //     .into()
}

fn solve(input: &str) -> (u32, usize) {
    let map = Matrix::from_string_lines(input).map(|c| {
        assert!(c.is_ascii_digit());
        *c as u32 - '0' as u32
    });
    let mut sol_a = 0;
    let mut low_points: Vec<Point> = Vec::new();
    // For each point in the map that's not height 9, the downhill neighbors.
    let mut flows_to: Matrix<Vec<Point>> = map.map(|_| Vec::new());
    let mut active: VecDeque<Point> = Default::default();
    let mut in_basin: Matrix<Option<Point>> = map.map(|_| None);

    // For the low points, the
    let mut basins: HashMap<Point, Vec<Point>> = HashMap::new();
    for (p, &v) in map.point_values() {
        let ns = map.neighbors4(p);
        if ns.iter().all(|(_np, nv)| **nv > v) {
            low_points.push(p);
            sol_a += v + 1;
            in_basin[p] = Some(p);
            basins.insert(p, vec![p]);
        } else if v < 9 {
            flows_to[p] = ns
                .iter()
                .filter(|(_, nv)| **nv < v)
                .map(|(np, _)| *np)
                .collect();
            active.push_back(p);
        }
    }

    while let Some(p) = active.pop_front() {
        assert!(in_basin[p].is_none());
        assert!(!flows_to[p].is_empty());
        let nbr_basins: Vec<Option<Point>> = flows_to[p].iter().map(|np| in_basin[*np]).collect();
        if nbr_basins.iter().all(Option::is_some) {
            // all downhills have been done; are they all the same?
            let nb = nbr_basins[0].unwrap();
            if nbr_basins.iter().all(|x| x.unwrap() == nb) {
                println!("resolved {p} is in basin of {nb}");
                in_basin[p] = Some(nb);
                basins.get_mut(&nb).unwrap().push(p);
            } else {
                println!("resolved {p} is not in a single basin");
            }
        } else {
            println!("try again later on {p}");
            active.push_back(p);
        }
    }

    let mut lens: Vec<usize> = basins.values().map(|b| b.len()).collect();
    lens.sort();
    let sol_b = lens.iter().rev().take(3).product();

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
}
