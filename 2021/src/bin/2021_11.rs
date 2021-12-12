// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/11

use aoclib::Matrix;

fn main() {
    let input = input();
    println!("{:?}", solve(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/11.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let mut m: Matrix<u32> = Matrix::from_digit_lines(input);
    let mut total_flashes = 0;
    let mut sol_a = None;
    for step in 1.. {
        m.update(|v| *v += 1);
        let mut flashed = Matrix::same_size(&m, false);
        loop {
            let mut found = false;
            for p in m.iter_points() {
                if m[p] > 9 && !flashed[p] {
                    flashed[p] = true;
                    total_flashes += 1;
                    found = true;
                    for q in m.neighbor8_points(p) {
                        m[q] += 1;
                    }
                }
            }
            if !found {
                break;
            }
        }
        for (p, &v) in flashed.point_values() {
            if v {
                m[p] = 0;
            }
        }
        if step == 100 {
            sol_a = Some(total_flashes);
        }
        if flashed.values().all(|&x| x) {
            return (sol_a.unwrap(), step);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve(&input), (1652, 220));
    }
}
