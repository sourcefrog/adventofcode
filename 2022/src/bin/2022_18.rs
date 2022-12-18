//! https://adventofcode.com/2022/day/18

use std::cmp::max;
use std::collections::HashSet;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/18.txt").unwrap()
}

fn parse(input: &str) -> HashSet<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn solve_a(input: &str) -> usize {
    let cubes = parse(input);
    cubes
        .iter()
        .flat_map(nbrs)
        .filter(|n| !cubes.contains(n))
        .count()
}

#[allow(clippy::ptr_arg)] // avoids needing lambdas in callers, and it's always a vec
fn nbrs(p: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut r = Vec::new();
    for dim in 0..=2 {
        for dir in [-1, 1] {
            let mut n = p.clone();
            n[dim] += dir;
            r.push(n)
        }
    }
    r
}

fn solve_b(input: &str) -> usize {
    let cubes = parse(input);
    let origin = vec![0, 0, 0];
    // x,y,z of a bounding box with one margin space.
    let mut bound = origin.clone();
    for c in &cubes {
        for dim in 0..=2 {
            bound[dim] = max(bound[dim], c[dim] + 1);
        }
    }
    // println!("{bound:?}");
    // Flood fill from 0,0,0 within a bounding box around all cubes.
    let mut steam: HashSet<Vec<isize>> = Default::default();
    // q is cubes known to contain steam that have not yet been explored.
    let mut q: Vec<Vec<isize>> = [origin].into();
    while let Some(s) = q.pop() {
        steam.insert(s.clone());
        'n: for n in nbrs(&s) {
            for dim in 0..=2 {
                if n[dim] > bound[dim] || n[dim] < -1 {
                    // don't travel too far
                    continue 'n;
                }
            }
            if !cubes.contains(&n) && !steam.contains(&n) && !q.contains(&n) {
                // println!("steam in {n:?}");
                q.push(n)
            }
        }
    }
    cubes
        .iter()
        .flat_map(nbrs)
        .filter(|n| steam.contains(n))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    static EX: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn ex1() {
        assert_eq!(solve_a(EX), 64);
    }

    #[test]
    fn ex2() {
        assert_eq!(solve_b(EX), 58);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 4504);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2556);
    }
}
