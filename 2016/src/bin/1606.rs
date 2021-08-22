//! https://adventofcode.com/2016/day/6

const DAY: &str = "1606";

fn solve_type_a(input: &str) -> String {
    let inp: Vec<Vec<u8>> = input
        .lines()
        .map(str::trim)
        .map(|l| l.bytes().collect())
        .collect();
    let mut r = String::new();
    for col in 0..8 {
        let mut counts = [0; 128];
        for row in &inp {
            counts[row[col] as usize] += 1;
        }
        let comm = counts
            .iter()
            .enumerate()
            .max_by_key(|(_i, count)| **count)
            .unwrap()
            .0;
        r.push(char::from_u32(comm as u32).unwrap());
    }
    r
}

fn solve_type_b(_input: &str) -> usize {
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
mod test1606 {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), "tsreykjj");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 0);
    }
}
