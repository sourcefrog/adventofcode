//! https://adventofcode.com/2016/day/6

const DAY: &str = "1606";

/// Returns a vec with 8 elements, one per column, each element
/// being an array of length 128 giving the frequency of that
/// ASCII character within that column.
fn frequency_tables(input: &str) -> Vec<[usize; 128]> {
    let mut f = vec![[0usize; 128]; 8];
    for l in input.lines() {
        assert_eq!(l.len(), f.len());
        for (col, ch) in l.trim().bytes().enumerate() {
            f[col][ch as usize] += 1
        }
    }
    f
}

fn solve_type_a(input: &str) -> String {
    let f = frequency_tables(input);
    let mut r = String::new();
    for counts in f {
        let most = counts
            .iter()
            .enumerate()
            .max_by_key(|(_i, count)| **count)
            .unwrap()
            .0;
        r.push(most as u8 as char);
    }
    r
}

fn solve_type_b(input: &str) -> String {
    let f = frequency_tables(input);
    let mut r = String::new();
    for counts in f {
        let least = counts
            .iter()
            .enumerate()
            .filter(|(_i, count)| **count > 0)
            .min_by_key(|(_i, count)| **count)
            .unwrap()
            .0;
        r.push(least as u8 as char);
    }
    r
}

fn input() -> String {
    std::fs::read_to_string(format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> String {
    solve_type_a(&input())
}

fn solve_b() -> String {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test1606 {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), "tsreykjj");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), "hnfbujie");
    }
}
