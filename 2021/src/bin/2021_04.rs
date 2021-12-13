// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/4

use bitvec::prelude::*;

use aoclib::Matrix;

fn main() {
    let (a, b) = solve(&input());
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/04.txt").unwrap()
}

/// True if any row or any column is all true.
fn has_won(hits: &Matrix<bool>) -> bool {
    hits.rows().any(|mut row| row.all(|h| *h)) || hits.columns().any(|mut col| col.all(|h| *h))
}

fn score(hit: &Matrix<bool>, mat: &Matrix<u32>, last_call: u32) -> u32 {
    hit.point_values()
        .filter(|(_p, v)| !*v)
        .map(|(p, _v)| mat[p])
        .sum::<u32>()
        * last_call
}

/// Parse out the list of called numbers and then a sequence of mats.
fn parse(input: &str) -> (Vec<u32>, Vec<Matrix<u32>>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let calls: Vec<u32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mats: Vec<Matrix<u32>> = lines[1..]
        .chunks(6)
        .map(|chunk| {
            chunk[1..]
                .iter()
                .map(|s| s.split_whitespace().map(|w| w.parse::<u32>().unwrap()))
                .collect()
        })
        .collect();
    (calls, mats)
}

fn solve(input: &str) -> (u32, u32) {
    let (calls, mats) = parse(input);
    let mut hits = vec![Matrix::new(5, 5, false); mats.len()];
    let mut done: BitVec = BitVec::repeat(false, mats.len());
    let mut part_a = None;
    for call in calls {
        for (mnum, mat) in mats.iter().enumerate() {
            if done[mnum] {
                continue;
            }
            if let Some(point) = mat.find(|x| *x == call) {
                hits[mnum][point] = true;
                // println!("found {} in mat {} at {:?}", call, mnum, p);
                if has_won(&hits[mnum]) {
                    if !done.any() {
                        part_a = score(&hits[mnum], mat, call).into();
                    }
                    // println!("winner!");
                    done.set(mnum, true);
                    if done.all() {
                        return (part_a.unwrap(), score(&hits[mnum], mat, call));
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve(&input()), (49860, 24628));
    }
}
