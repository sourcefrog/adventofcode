//! https://adventofcode.com/2021/day/4

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/04.txt").unwrap()
}

fn has_won(hits: &Matrix<bool>) -> bool {
    hits.rows().any(|mut r| r.all(|h| *h)) || hits.columns().any(|mut r| r.all(|h| *h))
}

fn score(hit: &Matrix<bool>, mat: &Matrix<u32>, last_call: u32) -> u32 {
    hit.point_values()
        .filter(|(_p, v)| !*v)
        .map(|(p, _v)| mat[p])
        .sum::<u32>()
        * last_call
}

fn solve_a(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let calls: Vec<u32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    dbg!(&calls);
    let mut mats: Vec<Matrix<u32>> = Vec::new();
    for chunk in lines[1..].chunks(6) {
        let m: Matrix<_> = chunk[1..]
            .iter()
            .map(|s| s.split_whitespace().map(|w| w.parse::<u32>().unwrap()))
            .collect();
        mats.push(m);
    }
    let mut hits: Vec<Matrix<bool>> = (0..mats.len()).map(|_| Matrix::new(5, 5, false)).collect();
    for i in calls {
        for (mnum, mat) in mats.iter().enumerate() {
            for (p, mm) in mat.point_values() {
                if *mm == i {
                    hits[mnum][p] = true;
                    println!("found {} in mat {} at {:?}", i, mnum, p);
                    if has_won(&hits[mnum]) {
                        println!("winner!");
                        return score(&hits[mnum], mat, i);
                    }
                }
            }
        }
    }
    unreachable!()
}

fn solve_b(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let calls: Vec<u32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    dbg!(&calls);
    let mut mats: Vec<Matrix<u32>> = Vec::new();
    for chunk in lines[1..].chunks(6) {
        let m: Matrix<_> = chunk[1..]
            .iter()
            .map(|s| s.split_whitespace().map(|w| w.parse::<u32>().unwrap()))
            .collect();
        mats.push(m);
    }
    let mut hits: Vec<Matrix<bool>> = (0..mats.len()).map(|_| Matrix::new(5, 5, false)).collect();
    let mut done = vec![false; mats.len()];
    for i in calls {
        for (mnum, mat) in mats.iter().enumerate() {
            for (p, mm) in mat.point_values() {
                if *mm == i {
                    hits[mnum][p] = true;
                    println!("found {} in mat {} at {:?}", i, mnum, p);
                    if has_won(&hits[mnum]) {
                        println!("winner!");
                        done[mnum] = true;
                        if done.iter().all(|b| *b) {
                            return score(&hits[mnum], mat, i);
                        }
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
    fn solution_a() {
        assert_eq!(solve_a(&input()), 49860);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 24628);
    }
}
