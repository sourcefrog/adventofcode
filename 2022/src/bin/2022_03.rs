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
    for ls in input.lines() {
        let l = ls.as_bytes();
        let ll = l.len();
        assert!(ll % 2 == 0);
        let a = &l[0..(ll / 2)];
        let b = &l[(ll / 2)..];
        for ca in a {
            if b.contains(ca) {
                sum += pri(*ca);
                break;
            }
        }
    }
    sum
}

fn pri(ca: u8) -> usize {
    (if ca.is_ascii_lowercase() {
        (ca - b'a') + 1
    } else {
        (ca - b'A') + 27
    }) as usize
}

fn solve_b(input: &str) -> usize {
    let mut sum = 0;
    for ll in input.lines().collect::<Vec<&str>>().chunks(3) {
        for ca in ll[0].as_bytes() {
            if ll[1].as_bytes().contains(ca) && ll[2].as_bytes().contains(ca) {
                sum += pri(*ca);
                break;
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
