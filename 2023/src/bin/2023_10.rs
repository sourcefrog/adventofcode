//! Pipe Maze <https://adventofcode.com/2023/day/10>

use std::fs::read_to_string;

// use itertools::Itertools;

use aoclib::Dir;
use aoclib::Matrix;

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

// enum Tile {
//     NS,
//     WE,
//     NE,
//     SE,
//     NW,
//     SW,
//     Ground,
//     Start,
// }

// impl Tile {
//     fn from_char(c: char) -> Self {
//         match c {
//             '|' => Self::NS,
//             '-' => Self::WE,
//             '.'=>Self::Ground,
//             'S'=>Self::Start,
//             'L'=>Self::NE,
//             'J' =>Self::NW,
//             '

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

fn solve_a(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let start_pos = map.find_single_value(&'S');
    dbg!(&start_pos);

    let mut cursor = start_pos;

    /* Trace the loop from the starting point. Since we don't know
    what connections go out from it, start by looking at its
    four neighbors to find one that connects _in_, and
     go from there.

     To traverse each pipe we need to remember which direction we're heading.
     */
    let mut distance = 0;
    let mut heading = None;
    for dir in Dir::iter() {
        let np = start_pos.step(dir);
        if can_enter(map[np], dir) {
            heading = Some(dir);
            distance = 1;
            cursor = np;
            break;
        }
    }
    let mut heading = heading.expect("initial heading"); // the direction that we were heading to move into the current cursor position
    dbg!(heading);

    while map[cursor] != 'S' {
        heading = traverse(map[cursor], heading);
        cursor = cursor.step(heading);
        distance += 1;
    }

    /* We found the distance around the loop, including the step back to the start, but what we
     * really want is the distance to the furthest tile, which would be half of that.
     *
     * If it was odd, we should round down, because you could choose to go the shorter of
     * two paths to any tile.
     */
    distance / 2
}

fn solve_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
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
}
