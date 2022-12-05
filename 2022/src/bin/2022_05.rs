//! https://adventofcode.com/2022/day/5

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/05.txt").unwrap()
}

fn solve_a(input: &str) -> String {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut lines = input.lines();
    while let Some(l) = lines.next() {
        if l.starts_with(" 1 ") {
            break;
        }
        for i in 0..9 {
            let c = l.chars().nth(1 + 4 * i).unwrap();
            assert!(c.is_ascii_uppercase() || c == ' ');
            if c.is_ascii_uppercase() {
                stacks[i].insert(0, c);
            }
        }
    }
    assert!(lines.next().unwrap().trim().is_empty());
    while let Some(l) = lines.next() {
        let w: Vec<&str> = l.split_ascii_whitespace().collect();
        let cnt: usize = w[1].parse().unwrap();
        let from_i: usize = w[3].parse::<usize>().unwrap() - 1;
        let to_i: usize = w[5].parse::<usize>().unwrap() - 1;
        // for _ in 0..cnt {
        let j = stacks[from_i].len() - cnt;
        let mut x = stacks[from_i].split_off(j);
        assert_eq!(x.len(), cnt);
        stacks[to_i].append(&mut x)
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
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
