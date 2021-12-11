// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/11

use aoclib::Matrix;

fn main() {
    let input = input();
    println!("{:?}", solve_a(&input));
    println!("{:?}", solve_b(&input));
}

fn input() -> String {
    std::fs::read_to_string("input/11.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut m: Matrix<u32> = Matrix::from_string_lines(input).map(|&c| {
        assert!(c.is_ascii_digit());
        c as u32 - '0' as u32
    });
    let mut tf = 0;
    for step in 0..100 {
        dbg!(&step);
        let mut n = m.map(|&v| v + 1);
        let mut flashed = m.map(|_| false);
        loop {
            // println!(
            //     "{}",
            //     n.map(|&v| "0123456789abcdefghijk".chars().nth(v as usize).unwrap())
            // );
            let mut done = true;
            for p in n.iter_points() {
                if n[p] > 9 && !flashed[p] {
                    flashed[p] = true;
                    tf += 1;
                    println!("{} flashed", p);
                    for q in n.neighbor8_points(p) {
                        n[q] += 1;
                        done = false;
                    }
                }
            }
            if done {
                break;
            }
        }
        for p in n.iter_points() {
            if flashed[p] {
                n[p] = 0;
            }
        }
        m = n;
    }
    tf
}

fn solve_b(input: &str) -> usize {
    let mut m: Matrix<u32> = Matrix::from_string_lines(input).map(|&c| {
        assert!(c.is_ascii_digit());
        c as u32 - '0' as u32
    });
    let mut tf = 0;
    for step in 1.. {
        dbg!(&step);
        let mut n = m.map(|&v| v + 1);
        let mut flashed = m.map(|_| false);
        loop {
            // println!(
            //     "{}",
            //     n.map(|&v| "0123456789abcdefghijk".chars().nth(v as usize).unwrap())
            // );
            let mut done = true;
            for p in n.iter_points() {
                if n[p] > 9 && !flashed[p] {
                    flashed[p] = true;
                    tf += 1;
                    println!("{} flashed", p);
                    for q in n.neighbor8_points(p) {
                        n[q] += 1;
                        done = false;
                    }
                }
            }
            if done {
                break;
            }
        }
        for p in n.iter_points() {
            if flashed[p] {
                n[p] = 0;
            }
        }
        m = n;
        if flashed.values().all(|&x| x) {
            return step;
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
        assert_eq!(solve_a(&input), 0);
        assert_eq!(solve_b(&input), 0);
    }
}
