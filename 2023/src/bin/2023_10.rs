//! Pipe Maze <https://adventofcode.com/2023/day/10>

use std::fs::read_to_string;

// use itertools::Itertools;

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

fn effective_s_char(map: &Matrix<char>, s_point: Point) -> char {
    use Dir::*;
    assert_eq!(map[s_point], 'S');
    let mut connected_dirs = vec![];
    for dir in Dir::iter() {
        let np = s_point.step(dir);
        if map.contains_point(np) && can_enter(map[np], dir) {
            connected_dirs.push(dir);
        }
    }
    assert_eq!(connected_dirs.len(), 2);
    connected_dirs.sort();
    match connected_dirs.as_slice() {
        [N, E] => 'L',
        [N, W] => 'J',
        [S, E] => 'F',
        [S, W] => '7',
        [N, S] => '|',
        [E, W] => '-',
        _ => unreachable!("{connected_dirs:?}"),
    }
}

fn solve_b(input: &str) -> usize {
    /* First, find the loop. */
    let map = Matrix::from_string_lines(input);
    let loop_points = trace_loop(&map);
    let mut pretty_map = map.map(|c| to_unicode_box(*c));
    let loop_map = Matrix::from_points_with_size(loop_points, map.width(), map.height());
    for point in pretty_map.points() {
        if !loop_map[point] {
            pretty_map[point] = '⋅';
        }
    }
    println!("{}", pretty_map.to_string_lines());

    /* Now we know this about the loop, since it _is_ a loop:
     *
     * There is only one loop on the map. It's closed. It does not cross itself.
     *
     * We can find the enclosed points by starting at the top and walking down, keeping
     * track of which columns are inside the loop.
     *
     * The structure of these characters means that different parts of one cell might be
     * inside the loop while another is outside: in fact for tiles on the loop this must
     * always be true. So, for containment, we track whether the top left corner of
     * the cell is contained. All the other edges can be inferred from this, given the
     * cell shape and the knowledge that it is part of the loop.
     *
     * The number of fully contained cells is then the number of cells that are not part
     * of the loop and whose top-left is contained.
     */
    let mut inside_cols = vec![false; map.width()];
    let mut contained = 0;
    for y in 0..map.height() {
        #[allow(clippy::needless_range_loop)]
        for x in 0..map.width() {
            let p = point(x as isize, y as isize);
            if loop_map[p] {
                let mut c = map[p];
                if c == 'S' {
                    c = effective_s_char(&map, p);
                }
                match c {
                    '-' | 'J' | '7' => inside_cols[x] ^= true,
                    _ => (),
                }
            } else if inside_cols[x] {
                pretty_map[p] = 'I';
                contained += 1;
            }
        }
    }
    println!("{}", pretty_map.to_string_lines());
    contained
}

/// Follow the loop from the starting point and return all the points on that loop,
/// finishing at the start.
fn trace_loop(map: &Matrix<char>) -> Vec<Point> {
    let start_pos = map.find_single_value(&'S');
    let mut loop_points = Vec::new();
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
            loop_points.push(np);
            break;
        }
    }
    let mut heading = heading.expect("initial heading"); // the direction that we were heading to move into the current cursor position
    dbg!(heading);

    while map[cursor] != 'S' {
        heading = traverse(map[cursor], heading);
        cursor = cursor.step(heading);
        loop_points.push(cursor);
    }
    loop_points
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 6820);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 337);
    }

    #[test]
    fn example_3() {
        let input = indoc! {"\
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        assert_eq!(solve_b(input), 8);
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
