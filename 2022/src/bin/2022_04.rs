//! https://adventofcode.com/2022/day/

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/04.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut count = 0;
    for l in input.lines() {
        let n: Vec<usize> = l
            .split(|c: char| !c.is_ascii_digit())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        assert_eq!(n.len(), 4);
        // if (n[0] <= n[2] && n[1] >= n[3]) || (n[2] <= n[0] && n[3] >= n[1]) {
        //     count += 1;
        // }
        if (n[1] >= n[2] && n[1] <= n[3])
            || (n[0] >= n[2] && n[0] <= n[3])
            || (n[2] >= n[0] && n[2] <= n[1])
            || (n[3] >= n[0] && n[3] <= n[1])
        {
            count += 1
        }
    }
    count
}

fn solve_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 0);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 0);
    }
}
