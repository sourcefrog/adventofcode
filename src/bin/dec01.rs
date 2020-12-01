use std::collections::HashSet;

pub fn main() {
    println!("dec01a: {}", solve_a());
    println!("dec01b: {}", solve_b());
}

fn solve_a() -> isize {
    let input = load_input();

    for a in &input {
        for b in &input {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("no combination found");
}

fn solve_b() -> isize {
    let vals: HashSet<isize> = load_input().into_iter().collect();
    for a in &vals {
        for b in &vals {
            let c = 2020 - a - b;
            if c > 0 && vals.contains(&c) {
                return a * b * c;
            }
        }
    }
    panic!("no combination found");
}

fn load_input() -> Vec<isize> {
    std::fs::read_to_string("input/dec01.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 646779);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 246191688);
    }
}
