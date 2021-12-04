//! https://adventofcode.com/2021/day/4

use bitvec::prelude::*;

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
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

fn solve_a(input: &str) -> u32 {
    let (calls, mats) = parse(input);
    let mut hits = vec![Matrix::new(5, 5, false); mats.len()];
    for call in calls {
        for (mnum, mat) in mats.iter().enumerate() {
            for (p, mm) in mat.point_values() {
                if *mm == call {
                    hits[mnum][p] = true;
                    // println!("found {} in mat {} at {:?}", call, mnum, p);
                    if has_won(&hits[mnum]) {
                        // println!("winner!");
                        return score(&hits[mnum], mat, call);
                    }
                    break;
                }
            }
        }
    }
    unreachable!()
}

fn solve_b(input: &str) -> u32 {
    let (calls, mats) = parse(input);
    let mut hits = vec![Matrix::new(5, 5, false); mats.len()];
    let mut done: BitVec = BitVec::repeat(false, mats.len());
    for call in calls {
        for (mnum, mat) in mats.iter().enumerate() {
            if done[mnum] {
                continue;
            }
            for (p, mm) in mat.point_values() {
                if *mm == call {
                    hits[mnum][p] = true;
                    // println!("found {} in mat {} at {:?}", call, mnum, p);
                    if has_won(&hits[mnum]) {
                        // println!("winner!");
                        done.set(mnum, true);
                        if done.all() {
                            return score(&hits[mnum], mat, call);
                        }
                    }
                    break;
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
    fn solution_a() {
        assert_eq!(solve_a(&input()), 49860);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 24628);
    }
}
