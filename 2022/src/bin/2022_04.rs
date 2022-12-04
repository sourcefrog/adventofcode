//! https://adventofcode.com/2022/day/

fn main() {
    let (a, b) = solve(&input());
    println!("{a}");
    println!("{b}");
}

fn input() -> String {
    std::fs::read_to_string("input/04.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let mut enclosed = 0;
    let mut overlap = 0;
    for l in input.lines() {
        let n: Vec<usize> = l
            .split(|c: char| !c.is_ascii_digit())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        assert_eq!(n.len(), 4);
        if (n[0] <= n[2] && n[1] >= n[3]) || (n[2] <= n[0] && n[3] >= n[1]) {
            enclosed += 1;
        }
        if (n[0] >= n[2] && n[0] <= n[3])
            || (n[1] >= n[2] && n[1] <= n[3])
            || (n[2] >= n[0] && n[2] <= n[1])
            || (n[3] >= n[0] && n[3] <= n[1])
        {
            overlap += 1
        }
    }
    (enclosed, overlap)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve(&input()), (433, 852));
    }
}
