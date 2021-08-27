//! https://adventofcode.com/2016/day/9

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
            let mut repbuf =  String::new(); 
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

fn solve_type_b(input: &str) -> usize {
    0
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
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
mod test1609 {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 70186);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 0);
    }
}
