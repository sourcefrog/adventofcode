//! https://adventofcode.com/2016/day/5

use md5;

const DAY: &str = "1605";

fn solve_type_a(input: &str) -> String {
    let input = input.trim();
    let mut r = String::new();
    for i in 0.. {
        let msg = format!("{}{}", input, i);
        let digest = md5::compute(msg.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 {
            r.push(char::from_digit((digest[2] & 0x0f) as u32, 16).unwrap());
            // dbg!(&r);
            if r.len() == 8 {
                return r;
            }
        }
    }
    unreachable!()
}

fn solve_type_b(input: &str) -> usize {
    0
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> String {
    solve_type_a(&input())
}

fn solve_b() -> usize {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), "c6697b55");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 0);
    }
}
