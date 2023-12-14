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
    let mut tot = 0;
    for block in input.split("\n\n") {
        let map = Matrix::from_string_lines(block).map(|c| *c == '#');
        dbg!(map.width(), map.height());
        tot += find_reflections(&map);
    }
    tot
}

fn find_reflections(map: &Matrix<bool>) -> usize {
    let h = map.height();
    let w = map.width();

    println!("{}", map.to_string_lines());

    'col: for xmirror in 1..w {
        for x1 in 0..xmirror {
            let x2 = (xmirror - x1) + xmirror - 1;
            println!("{xmirror}: check col {x1} against {x2}");
            if x2 < w && !map.column(x1).eq(map.column(x2)) {
                println!("   >mismatch");
                continue 'col;
            }
        }
        println!("found reflection at x={xmirror}");
        return xmirror;
    }

    'row: for ymirror in 1..h {
        for y in 0..=ymirror {
            let y2 = (ymirror - y) + (ymirror - 1);
            println!("{ymirror}: check {y} against {y2}");
            if y2 < h && !map.row(y).eq(map.row(y2)) {
                continue 'row;
            }
        }
        println!("found reflection at y={ymirror}");
        return 100 * ymirror;
    }
    unreachable!("No reflection found in\n{}", map.to_string_lines());
}

fn solve_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"\
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        " };

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE), 405);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 27505);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
