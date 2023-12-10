//! Pipe Maze <https://adventofcode.com/2023/day/10>

use std::fs::read_to_string;

use itertools::Itertools;

use aoclib::Dir;
use aoclib::Matrix;

use aoclib::point;
use aoclib::Point;
use strum::IntoEnumIterator;

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

/// For all the cell types, what are the two _exit_ directions.
static TILES: &[(char, [Dir; 2])] = &[
    ('|', [Dir::N, Dir::S]),
    ('-', [Dir::W, Dir::E]),
    ('L', [Dir::N, Dir::E]),
    ('F', [Dir::S, Dir::E]),
    ('7', [Dir::S, Dir::W]),
    ('J', [Dir::N, Dir::W]),
];

/// True if you can enter this cell heading in the given direction.
fn can_enter(c: char, dir: Dir) -> bool {
    if c == '.' {
        return false;
    }
    for (tc, tdir) in TILES {
        if c == *tc {
            return tdir.contains(&dir.invert());
        }
    }
    panic!("({c:?}, {dir:?})");
}

/// If we enter this cell in the given direction in which direction
/// will we leave?
fn traverse(c: char, dir: Dir) -> Dir {
    for (tc, tdirs) in TILES {
        if c == *tc {
            for i in 0..2 {
                if tdirs[i] == dir.invert() {
                    return tdirs[1 - i];
                }
            }
            panic!("didn't find path through {c:?} for {dir:?}");
        }
    }
    panic!("({c:?}, {dir:?})");
}

fn to_unicode_box(c: char) -> char {
    match c {
        '-' => '─',
        '|' => '│',
        'L' => '└',
        'F' => '┌',
        'J' => '┘',
        '7' => '┐',
        '.' => '⋅',
        _ => c,
    }
}

fn solve_a(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let distance = trace_loop(&map).len();

    /* We found the distance around the loop, including the step back to the start, but what we
     * really want is the distance to the furthest tile, which would be half of that.
     *
     * If it was odd, we should round down, because you could choose to go the shorter of
     * two paths to any tile.
     */
    distance / 2
}

fn solve_b(input: &str) -> usize {
    /* First, find the loop. */
    let map = Matrix::from_string_lines(input);
    let trace = trace_loop(&map);
    let loop_points = trace.iter().map(|(p, _dir)| *p).collect_vec();
    let mut pretty_map = map.map(|c| to_unicode_box(*c));
    for point in pretty_map.points() {
        if !loop_points.contains(&point) {
            pretty_map[point] = '⋅';
        }
    }
    let mut loop_map = map.map(|_| false);
    for p in &loop_points {
        loop_map[*p] = true
    }
    println!("{}", pretty_map.to_string_lines());

    /* Another idea:
     *
     * We can know, from tracing the loop, which orientation we're heading at any time,
     * and we can collect any non-loop squares to either the left or the right side.
     *
     * We don't know, apriori, whether the loop will be counterclockwise or clockwise,
     * but we could just try both. In one of them, at some point on the loop, tracing
     * out to the side will reach the edge, and we can discard that option.
     */
    for which_side in [true, false] {
        let mut touched_side = false;
        for (p, heading) in &trace {
            let side_dir = if which_side {
                heading.turn_right()
            } else {
                heading.turn_left()
            };
            let mut np = *p;
            loop {
                np = np.step(side_dir);
                if !loop_map.contains_point(np) {
                    touched_side = true;
                    break;
                } else if loop_map[np] {
                    break;
                } else {
                    pretty_map[np] = if which_side { 'R' } else { 'L' };
                }
            }
        }
    }

    println!("{}", pretty_map.to_string_lines());
    0
    // 7262 is too high?
}

/// Follow the loop from the starting point and return all the points on that loop,
/// finishing at the start, along with the direction we travelled to get to that point.
fn trace_loop(map: &Matrix<char>) -> Vec<(Point, Dir)> {
    let start_pos = map.find_single_value(&'S');
    let mut trace = Vec::new();
    let mut cursor = start_pos;

    /* Trace the loop from the starting point. Since we don't know
     * what connections go out from it, start by looking at its
     * four neighbors to find one that connects _in_, and
     * go from there.
     *
     * To traverse each pipe we need to remember which direction we're heading.
     */
    let mut heading = None;
    for dir in Dir::iter() {
        let np = start_pos.step(dir);
        if map.contains_point(np) && can_enter(map[np], dir) {
            heading = Some(dir);
            cursor = np;
            trace.push((cursor, dir));
            break;
        }
    }
    let mut heading = heading.expect("initial heading"); // the direction that we were heading to move into the current cursor position
    dbg!(heading);

    while map[cursor] != 'S' {
        trace.push((cursor, heading));
        heading = traverse(map[cursor], heading);
        cursor = cursor.step(heading);
    }
    trace.push((cursor, heading));
    trace
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn example_2() {
        let input = "\
";
        assert_eq!(solve_b(input), 0);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 6820);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }

    #[test]
    fn example_4() {
        let input = indoc! {"\
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(solve_b(input), 10);
    }
}
