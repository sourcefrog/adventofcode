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
        let mut found = None;
        'ca: for ca in a {
            for cb in b {
                if ca == cb {
                    assert!(
                        found.is_none() || found == Some(*ca),
                        "found twice in {ls} {ca} {cb}"
                    );
                    found = Some(*ca);
                    sum += pri(*ca);
                    break 'ca;
                }
            }
        }
        assert!(found.is_some());
        assert_eq!(a.len(), b.len());
    }
    sum
}

fn pri(ca: u8) -> usize {
    match ca {
        b'a'..=b'z' => (ca - b'a' + 1) as usize,
        b'A'..=b'Z' => (ca - b'A' + 27) as usize,
        x => panic!("unexpected char {}", x),
    }
}

fn solve_b(input: &str) -> usize {
    let mut sum = 0;
    for ll in input.lines().collect::<Vec<&str>>().chunks(3) {
        'chars: for c in ('a'..='z').chain('A'..='Z') {
            let c = c as u8;
            for l in ll {
                if !l.as_bytes().contains(&c) {
                    continue 'chars;
                }
            }
            sum += pri(c);
            break;
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
