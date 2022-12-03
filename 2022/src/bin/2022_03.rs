//! https://adventofcode.com/2022/day/3

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/03.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut sum: usize = 0;
    'line: for l in input.lines() {
        let ll = l.len();
        assert!(ll % 2 == 0);
        let (a, b) = l.split_at(ll / 2);
        for ca in a.chars() {
            if b.contains(ca) {
                sum += pri(ca);
                continue 'line;
            }
        }
    }
    sum
}

fn pri(ca: char) -> usize {
    (if ca.is_ascii_lowercase() {
        (ca as u8 - b'a') + 1
    } else {
        (ca as u8 - b'A') + 27
    }) as usize
}

fn solve_b(input: &str) -> usize {
    let mut sum = 0;
    'line: for ll in input.lines().collect::<Vec<&str>>().chunks(3) {
        for ca in ll[0].chars() {
            if ll[1].contains(ca) && ll[2].contains(ca) {
                sum += pri(ca);
                continue 'line;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 8039);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2510);
    }
}
