//! https://adventofcode.com/2022/day/18

use std::collections::HashSet;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/18.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let cubes: HashSet<Vec<isize>> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let mut open = 0;
    for c in &cubes {
        for dim in 0..=2 {
            for dir in &[-1, 1] {
                let mut n = c.clone();
                n[dim] += dir;
                if !cubes.contains(&n) {
                    open += 1;
                }
            }
        }
    }
    open
}

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
    let cubes: HashSet<Vec<isize>> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();
    let mut bound = vec![0, 0, 0];
    for c in &cubes {
        for dim in 0..=2 {
            bound[dim] = std::cmp::max(bound[dim], c[dim] + 1);
        }
    }
    // println!("{bound:?}");
    // Flood fill from 0,0,0 within a bounding box around all cubes.
    let mut steam: HashSet<Vec<isize>> = Default::default();
    // New is cubes known to contain steam that have not yet been explored.
    let mut new: Vec<Vec<isize>> = [vec![0, 0, 0]].into();
    while let Some(s) = new.pop() {
        steam.insert(s.clone());
        'n: for n in nbrs(&s) {
            for dim in 0..=2 {
                if n[dim] > bound[dim] || n[dim] < -1 {
                    // don't travel too far
                    continue 'n;
                }
            }
            if !cubes.contains(&n) && !steam.contains(&n) && !new.contains(&n) {
                // println!("steam in {n:?}");
                new.push(n)
            }
        }
    }
    let mut open = 0;
    for c in &cubes {
        for dim in 0..=2 {
            for dir in &[-1, 1] {
                let mut n = c.clone();
                n[dim] += dir;
                if !cubes.contains(&n) && steam.contains(&n) {
                    open += 1;
                }
            }
        }
    }
    open
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
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
