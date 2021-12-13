//! https://adventofcode.com/2021/day/2

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/02.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    for l in input.lines() {
        let (cmd, n) = l.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        match cmd {
            "forward" => x += n,
            "down" => y += n,
            "up" => y -= n,
            _ => panic!(),
        }
    }
    x * y
}

fn solve_b(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for l in input.lines() {
        let (cmd, n) = l.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();
        match cmd {
            "forward" => {
                x += n;
                y += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!(),
        }
    }
    x * y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1524750);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 1592426537);
    }
}
