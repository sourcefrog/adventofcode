//! https://adventofcode.com/2022/day/9

use aoclib::{point, Matrix};

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/09.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    solve(input, 2)
}

fn solve_b(input: &str) -> usize {
    solve(input, 10)
}

fn solve(input: &str, rope_len: usize) -> usize {
    // a guess at how much space is enough...
    // if it was wrong we'd panic when a point goes outside of the matrix,
    // and we don't.
    let mut vis = Matrix::new(1000, 1000, false);
    let mut knots = vec![point(500, 500); rope_len];
    vis[knots[0]] = true;

    for l in input.lines() {
        let dir = l.chars().next().unwrap();
        let steps = l.split_at(2).1.parse::<usize>().unwrap();
        for _ in 0..steps {
            let h = knots[0];
            knots[0] = match dir {
                'U' => h.up(),
                'D' => h.down(),
                'L' => h.left(),
                'R' => h.right(),
                _ => panic!(),
            };
            // I looked for a `windows_mut`, but it can't be an iterator
            // because of reasons in
            // <https://users.rust-lang.org/t/iterator-over-mutable-windows-of-slice/17110/4>.
            for i in 0..(rope_len - 1) {
                let dx = knots[i].x - knots[i + 1].x;
                let dy = knots[i].y - knots[i + 1].y;
                if dx.abs() > 1 || dy.abs() > 1 {
                    knots[i + 1].x += dx.signum();
                    knots[i + 1].y += dy.signum();
                }
            }
            vis[knots[rope_len - 1]] = true;
        }
    }

    vis.values().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            super::solve_a(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            13
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 6284);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2661);
    }
}
