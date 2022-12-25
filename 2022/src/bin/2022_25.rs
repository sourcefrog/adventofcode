//! https://adventofcode.com/2022/day/25

fn main() {
    println!("{}", solve_a(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/25.txt").unwrap()
}

fn from_snafu(s: &str) -> isize {
    let mut a = 0;
    let mut x = 1;
    for (_, c) in s.chars().rev().enumerate() {
        let d = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("{c:?}"),
        };
        a += d * x;
        x *= 5;
    }
    a
}

fn to_snafu(mut x: isize) -> String {
    let mut s = String::new();
    loop {
        let mut d = x % 5;
        if d > 2 {
            d -= 5;
            x += 5;
        }
        x /= 5;
        s.insert(
            0,
            match d {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!("{d}"),
            },
        );
        if x == 0 {
            break;
        }
    }
    s
}

fn solve_a(input: &str) -> String {
    let s = input.lines().map(from_snafu).sum::<isize>();
    // println!("{s}");
    to_snafu(s)
}

#[allow(dead_code)]
static EX: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), "2==0=0===02--210---1");
    }
}
