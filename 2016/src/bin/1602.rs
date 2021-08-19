//! https://adventofcode.com/2016/day/2

fn solve_a() -> String {
    solve_type_a(&input())
}

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

fn input() -> String {
    std::fs::read_to_string("input/1602.txt").unwrap()
}

fn main() {
    println!("1602a: {}", solve_a());
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
    fn solution_a() {
        assert_eq!(solve_a(), "53255");
    }
}
