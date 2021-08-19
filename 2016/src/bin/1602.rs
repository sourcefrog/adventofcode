//! https://adventofcode.com/2016/day/2

use std::collections::HashMap;

fn solve_type_a(s: &str) -> String {
    let mut pos = 5;
    let mut code = String::new();
    for l in s.lines() {
        for c in l.trim().chars() {
            pos = match c {
                'U' => {
                    if pos <= 3 {
                        pos
                    } else {
                        pos - 3
                    }
                }
                'D' => {
                    if pos >= 7 {
                        pos
                    } else {
                        pos + 3
                    }
                }
                'L' => {
                    if pos % 3 == 1 {
                        pos
                    } else {
                        pos - 1
                    }
                }
                'R' => {
                    if pos % 3 == 0 {
                        pos
                    } else {
                        pos + 1
                    }
                }
                _ => panic!("unexpected char {}", c),
            };
            assert!((1..=9).contains(&pos), "invalid position {}", pos);
        }
        code.push(char::from_digit(pos, 10).unwrap());
    }
    code
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

fn solve_type_b(s: &str) -> String {
    let mut code = String::new();

    const BMAP: &str = "
  1
 234
56789
 ABC
  D
";

    // First, build a map from (x,y) coordinates to keys. y runs down.
    let mut bmap: HashMap<Position, char> = HashMap::new();
    for (y, l) in BMAP.lines().enumerate() {
        for (x, c) in l.trim_end().chars().enumerate() {
            if !c.is_whitespace() {
                bmap.insert(
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                    c,
                );
            }
        }
    }

    let mut pos = Position { x: 0, y: 3 };
    debug_assert_eq!(*bmap.get(&pos).unwrap(), '5');
    for l in s.lines() {
        for c in l.trim().chars() {
            let Position { mut x, mut y } = pos;
            match c {
                'U' => y -= 1,
                'D' => y += 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => panic!("unexpected char {}", c),
            };
            let candidate = Position { x, y };
            if bmap.contains_key(&candidate) {
                // it exists, move there. otherwise don't move.
                pos = candidate;
                // println!("move {} to {}", c, *bmap.get(&pos).unwrap());
            } else {
                // println!("can't move {} from {}", c, *bmap.get(&pos).unwrap());
            }
        }
        code.push(*bmap.get(&pos).expect("pos is on a button"));
    }
    code
}

fn input() -> String {
    std::fs::read_to_string("input/1602.txt").unwrap()
}

fn solve_a() -> String {
    solve_type_a(&input())
}

fn solve_b() -> String {
    solve_type_b(&input())
}

fn main() {
    println!("1602a: {}", solve_a());
    println!("1602b: {}", solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn example_a() {
        assert_eq!(solve_type_a(EXAMPLE), "1985");
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_type_b(EXAMPLE), "5DB3");
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), "53255");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), "7423A");
    }
}
