use std::collections::HashSet;
use std::fs::read_to_string;

use aoclib::Matrix;
use itertools::Itertools;

static PUZZLE: &str = env!("CARGO_BIN_NAME");

fn main() {
    let input = &input();
    println!("{PUZZLE} a {}", solve_a(input));
    println!("{PUZZLE} b {}", solve_b(input));
}

fn input() -> String {
    let (year, day) = PUZZLE.split_once('_').unwrap();
    read_to_string(format!("{year}/input/{day}.txt"))
        .or_else(|_| read_to_string(format!("input/{day}.txt")))
        .unwrap()
}

fn solve_a(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let stars = map.find_values(&'#').collect_vec();
    // dbg!(&stars);
    let star_xs: HashSet<isize> = stars.iter().map(|p| p.x).unique().collect();
    let star_ys: HashSet<isize> = stars.iter().map(|p| p.y).unique().collect();
    /* Between each distinct pair (regardless of order), count rows and columns, and count
     * twice any row or column with no stars. */
    /* Instead of iterating X and Y values, it would probably be faster to compute
     * once the expanded X and Y values, adjusting the star positions, and then we
     * can take the Manhattan distance directly. */
    let mut total = 0;
    for s1 in stars.iter() {
        for s2 in stars.iter() {
            let mut dist = 0;
            /* Count only to the right and down, which will have the effect of
             * counting each pair once. */
            for x in s1.x..s2.x {
                dist += 1 + (!star_xs.contains(&x)) as usize;
            }
            for y in s1.y..s2.y {
                dist += 1 + (!star_ys.contains(&y)) as usize;
            }
            total += dist;
        }
    }
    total
}

fn solve_b(input: &str) -> usize {
    0
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
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9403026)
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
