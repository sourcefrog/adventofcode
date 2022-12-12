//! https://adventofcode.com/2022/day/12

use aoclib::shortest_path::ShortestPath;
use aoclib::Matrix;
use itertools::Itertools;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/12.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let start = map.point_values().find(|(p, c)| **c == 'S').unwrap().0;
    let end = map.point_values().find(|(p, c)| **c == 'E').unwrap().0;
    dbg!(start, end);
    let path = ShortestPath::find(
        &start,
        |p| *p == end,
        |p| {
            map.neighbors4(*p)
                .flat_map(|(q, c)| {
                    let mut a = map[*p];
                    if a == 'S' {
                        a = 'a'
                    };
                    let mut c = *c;
                    if c == 'E' {
                        c = 'z'
                    };
                    if (c as u16) <= (a as u16 + 1) {
                        Some((q, 1))
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
    );
    // not 406
    path.distance()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
