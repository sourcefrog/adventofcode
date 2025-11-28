//! Cosmic Expansion
//!
//! <https://adventofcode.com/2023/day/11>

use std::cmp::max;

use aoclib::Matrix;
use itertools::Itertools;

static PUZZLE: &str = env!("CARGO_BIN_NAME");

fn main() {
    let input = &input();
    println!("{PUZZLE} a {}", solve_a(input));
    println!("{PUZZLE} b {}", solve_b(input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve_a(input: &str) -> isize {
    solve(input, 2)
}

fn solve_b(input: &str) -> isize {
    solve(input, 1_000_000)
}

/// Solve the general case, with each empty row/column made larger by a given factor.
fn solve(input: &str, expansion: isize) -> isize {
    /* All we really need is the set of x and y values to calculate the Manhattan distances: the point identities don't actually matter.  */
    let stars = Matrix::from_string_lines(input)
        .find_values(&'#')
        .collect_vec();
    let expanded_xs = expand(stars.iter().map(|p| p.x), expansion);
    let expanded_ys = expand(stars.iter().map(|p| p.y), expansion);
    expanded_xs
        .iter()
        .tuple_combinations()
        .map(|(a, b)| max(b - a, 0))
        .sum::<isize>()
        + expanded_ys
            .iter()
            .tuple_combinations()
            .map(|(a, b)| max(b - a, 0))
            .sum::<isize>()
}

fn expand(xs: impl IntoIterator<Item = isize>, expansion: isize) -> Vec<isize> {
    /* Increase all the xs to allow for blank columns. There may be duplicates. */
    assert!(expansion >= 1);
    let mut last_x = None;
    let mut offset = 0;
    let mut expanded_xs = Vec::new();
    for x in xs.into_iter().sorted() {
        if let Some(last_x) = last_x {
            offset += (expansion - 1) * max(x - last_x - 1, 0);
        }
        expanded_xs.push(x + offset);
        last_x = Some(x);
    }
    expanded_xs
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! { "\
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE), 374);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve(EXAMPLE, 10), 1030);
        assert_eq!(solve(EXAMPLE, 100), 8410);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9403026)
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 543018317006);
    }
}
