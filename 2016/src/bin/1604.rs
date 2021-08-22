//! https://adventofcode.com/2016/day/4

const DAY: &str = "1604";

fn letter_to_ord(c: char) -> usize {
    ((c as u32) - ('a' as u32)) as usize
}

fn ord_to_letter(o: usize) -> char {
    char::from_u32(('a' as u32) + (o as u32)).unwrap()
}

/// Return the five most common letters in s, in order by frequency,
/// with ties broken by alphabetical order. Non-letter characters are
/// ignored.
fn checksum(s: &str) -> String {
    let mut counts = [0; 26];
    for c in s.chars().filter(char::is_ascii_lowercase) {
        counts[letter_to_ord(c)] += 1;
    }
    let mut v: Vec<(isize, char)> = counts
        .iter()
        .enumerate()
        .map(|(ord, count)| (-(*count as isize), ord_to_letter(ord)))
        .collect();
    v.sort_unstable();
    v.iter().map(|(_count, c)| *c).take(5).collect()
}

fn solve_type_a(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            let (a, cksum) = l.strip_suffix(']').unwrap().split_once('[').unwrap();
            let (name, sector) = a.rsplit_once('-').unwrap();
            if cksum == checksum(name) {
                Some(sector.parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn solve_type_b(input: &str) -> usize {
    todo!()
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
mod test {
    use super::*;

    #[test]
    fn examples_a_parts() {
        assert_eq!(checksum("aaaaa-bbb-z-y-x-123"), "abxyz");
        assert_eq!(checksum("a-b-c-d-e-f-g-h-987"), "abcde");
        assert_eq!(checksum("not-a-real-room-404"), "oarel");
        assert_ne!(checksum("totally-real-room-200"), "decoy");
    }

    #[test]
    fn examples_a_whole() {
        assert_eq!(
            solve_type_a(
                "\
        aaaaa-bbb-z-y-x-123[abxyz]
        a-b-c-d-e-f-g-h-987[abcde]
        not-a-real-room-404[oarel]
        totally-real-room-200[decoy]"
            ),
            1514
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 245102);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(), 1921);
    }
}
