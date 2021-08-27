//! https://adventofcode.com/2016/day/9

use num_bigint::BigUint;

const DAY: &str = "1609";

fn read_int<I: Iterator<Item = char>>(inch: &mut I, terminator: char) -> usize {
    let mut r = 0;
    loop {
        let c = inch.next().unwrap();
        if c == terminator {
            break;
        } else if let Some(d) = char::to_digit(c, 10) {
            r = r * 10 + d
        } else {
            panic!("unexpected char {:#?}", c)
        }
    }
    r as usize
}

fn decompress(input: &str) -> String {
    let mut inch = input.chars().filter(|c| !c.is_ascii_whitespace());
    let mut o = String::new();
    while let Some(c) = inch.next() {
        if c == '(' {
            let replen = read_int(&mut inch, 'x');
            let repcount = read_int(&mut inch, ')');
            let mut repbuf = String::new();
            for _ in 0..replen {
                repbuf.push(inch.next().expect("string contains repeated content"));
            }
            for _ in 0..repcount {
                o.push_str(&repbuf)
            }
        } else {
            o.push(c)
        }
    }
    o
}

fn solve_type_a(input: &str) -> usize {
    decompress(input).len()
}

/// Return the decompressed length for a string that might contain recursive repetitions.
fn decompressed_len(input: &str) -> BigUint {
    let b: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    recurse_len(&b)
}

fn parse_int(b: &[char]) -> usize {
    b.iter().collect::<String>().parse().expect("parse int")
}

fn parse_repeat(b: &[char]) -> (usize, usize, &[char]) {
    let xpos = b.iter().position(|c| *c == 'x').expect("find x");
    let ppos = b.iter().position(|c| *c == ')').expect("find ')'");
    let replen = parse_int(&b[..xpos]);
    let repcnt = parse_int(&b[(xpos + 1)..ppos]);
    (replen, repcnt, &b[(ppos + 1)..])
}

fn recurse_len(mut b: &[char]) -> BigUint {
    let mut l = BigUint::default();
    // let orig_b = b;
    while !b.is_empty() {
        let c = b[0];
        b = &b[1..];
        if c == '(' {
            let (replen, repcnt, rest) = parse_repeat(&b);
            // println!("{}, {}", replen, repcnt);
            let sub_b = &rest[..replen];
            b = &rest[replen..];
            l += repcnt * recurse_len(sub_b);
        } else {
            l += 1u32
        }
    }
    // println!("{} => {}", orig_b.iter().collect::<String>(), l);
    l
}

fn solve_type_b(input: &str) -> BigUint {
    decompressed_len(input)
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> BigUint {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test1609 {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 70186);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_type_b("X(8x2)(3x3)ABCY"), 20u32.into());
        assert_eq!(
            solve_type_b("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920u32.into()
        );
        assert_eq!(
            solve_type_b("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445u32.into()
        );
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 10915059201u64.into());
    }
}
