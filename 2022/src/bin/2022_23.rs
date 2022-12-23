//! https://adventofcode.com/2022/day/23

use std::collections::{BTreeMap, BTreeSet};

use aoclib::{point, Point};

fn main() {
    // println!("{}", solve_a(SMOL));
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/23.txt").unwrap()
}

type Map = BTreeSet<Point>;

#[allow(dead_code)]
fn show(elvs: &BTreeSet<Point>) -> String {
    let (b0, b1) = bounds(elvs);
    show_fixed(elvs, b0.delta(-2, -2), b1.delta(2, 2))
}

#[allow(dead_code)]
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
    // println!("{}", input);
    // println!("{}", show(&elvs));
    let (elvs, stalled) = run1(elvs.clone(), 10);
    assert!(stalled.is_none());
    let (bb0, bb1) = bounds(&elvs);
    // println!("bounds {bb0:?}..{bb1:?}");
    (bb1.y - bb0.y + 1) * (bb1.x - bb0.x + 1) - elvs.len() as isize
    // not 3910
}

/// Returns the last map, and if it stalled the point at which it stalled.
fn run1(mut elvs: Map, nrounds: usize) -> (Map, Option<usize>) {
    let mut dir = 0;
    let n_elvs = elvs.len();
    for round in 1..=nrounds {
        // println!("round {round}");
        // From destination point to a list of elves considering moving there.
        let mut prop: BTreeMap<Point, Vec<Point>> = BTreeMap::new();
        'elvs: for &s in &elvs {
            if s.neighbors8().into_iter().all(|p| !elvs.contains(&p)) {
                // println!("{s:?} is lonely");
                continue;
            }

            for idir in 0..4 {
                let mdir = (dir + idir) % 4;
                // let dirch = "NSWE".chars().nth(mdir).unwrap();
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
                    // println!("{s:?} proposes to move {dirch} to {dst:?}");
                    continue 'elvs;
                } else {
                    // println!("{s:?} is blocked from moving {dirch}");
                }
            }
        }
        let mut any_moves = false;
        for (dst, ss) in prop.iter() {
            assert!(ss.len() >= 1);
            if ss.len() == 1 {
                // println!("{} moves to {dst:?}", ss[0]);
                assert!(elvs.remove(&ss[0]));
                assert!(elvs.insert(*dst));
                any_moves = true;
            } else {
                // println!("contention on {dst:?}: {} elves can't move", ss.len());
            }
        }

        // println!("{}", show(&elvs));
        assert_eq!(elvs.len(), n_elvs);
        dir = (dir + 1) % 4;
        if !any_moves {
            // println!("nobody moved in round {round}");
            return (elvs, Some(round));
        }
    }
    (elvs, None)
}

fn solve_b(input: &str) -> usize {
    let elvs = parse(input);
    // println!("{}", input);
    // println!("{}", show(&elvs));
    let (_last_map, rounds) = run1(elvs.clone(), 100000);
    rounds.expect("did not settle")
}

#[allow(dead_code)]
static EX: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

#[allow(dead_code)]
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
        assert_eq!(solve_a(EX), 110);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 4070);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 881);
    }
}
