//! https://adventofcode.com/2022/day/23

use std::collections::{BTreeMap, BTreeSet};

use aoclib::{point, Point};

fn main() {
    // println!("{}", solve_a(SMOL));
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/23.txt").unwrap()
}

type Map = BTreeSet<Point>;

fn show(elvs: &BTreeSet<Point>) -> String {
    let (b0, b1) = bounds(elvs);
    show_fixed(elvs, b0.delta(-2, -2), b1.delta(2, 2))
}

fn show_fixed(elvs: &BTreeSet<Point>, b0: Point, b1: Point) -> String {
    let mut s = String::new();
    for y in (b0.y)..=(b1.y) {
        for x in (b0.x)..=(b1.x) {
            s.push(if elvs.contains(&point(x, y)) {
                '#'
            } else {
                '.'
            });
        }
        s.push('\n');
    }
    s
}

fn parse(input: &str) -> Map {
    let mut y = 0;
    let mut m = BTreeSet::new();
    for l in input.lines() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                m.insert(point(x as isize, y));
            } else {
                assert_eq!(c, '.');
            }
        }
        y += 1;
    }
    m
}

fn bounds(map: &Map) -> (Point, Point) {
    let minx = map.iter().map(|p| p.x).min().unwrap();
    let miny = map.iter().map(|p| p.y).min().unwrap();
    let maxx = map.iter().map(|p| p.x).max().unwrap();
    let maxy = map.iter().map(|p| p.y).max().unwrap();
    (point(minx, miny), point(maxx, maxy))
}

fn solve_a(input: &str) -> isize {
    // let mat = aoclib::Matrix::from_string_lines(input);
    // println!("{mat}");
    let elvs = parse(input);
    println!("{}", input);
    println!("{}", show(&elvs));
    let maps = run1(elvs.clone());
    let elvs = maps.last().unwrap();
    let (bb0, bb1) = bounds(elvs);
    println!("bounds {bb0:?}..{bb1:?}");
    (bb1.y - bb0.y + 1) * (bb1.x - bb0.x + 1) - elvs.len() as isize
    // not 3910
}

fn run1(mut elvs: Map) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut dir = 0;
    let n_elvs = elvs.len();
    for round in 1..=10 {
        println!("round {round}");
        // From destination point to a list of elves considering moving there.
        let mut prop: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
        'elvs: for &s in &elvs {
            if s.neighbors8().into_iter().all(|p| !elvs.contains(&p)) {
                println!("{s:?} is lonely");
                continue;
            }

            for idir in 0..4 {
                let mdir = (dir + idir) % 4;
                let dirch = "NSWE".chars().nth(mdir).unwrap();
                let look_at: [Point; 3] = match mdir {
                    0 => [s.up(), s.up().left(), s.up().right()],
                    1 => [s.down(), s.down().left(), s.down().right()],
                    2 => [s.left(), s.left().up(), s.left().down()],
                    3 => [s.right(), s.right().up(), s.right().down()],
                    _ => panic!("{dir}"),
                };
                if look_at.iter().all(|l| !elvs.contains(l)) {
                    let dst = look_at[0];
                    prop.entry(dst).or_default().push(s);
                    println!("{s:?} proposes to move {dirch} to {dst:?}");
                    continue 'elvs;
                } else {
                    println!("{s:?} is blocked from moving {dirch}");
                }
            }
        }
        for (dst, ss) in prop.iter() {
            assert!(ss.len() >= 1);
            if ss.len() == 1 {
                println!("{} moves to {dst:?}", ss[0]);
                assert!(elvs.remove(&ss[0]));
                assert!(elvs.insert(*dst));
            } else {
                println!("contention on {dst:?}: {} elves can't move", ss.len());
            }
        }

        println!("{}", show(&elvs));
        assert_eq!(elvs.len(), n_elvs);
        maps.push(elvs.clone());
        dir = (dir + 1) % 4;
    }
    maps
}

fn solve_b(input: &str) -> usize {
    input.len()
}

static EX: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

static SMOL: &str = "\
.....
..##.
..#..
.....
..##.
.....
";
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex() {
        let maps = run1(parse(EX));
        assert_eq!(
            show_fixed(&maps[0], point(-2, -2), point(12, 12)),
            "\
..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............
"
        );
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 9900);
    // }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);
    // }
}
