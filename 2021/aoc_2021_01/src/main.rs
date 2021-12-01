fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let m: Vec<u32> = input.lines().map(str::parse).map(Result::unwrap).collect();
    m.windows(2)
        .filter(|i| {
            if let [a, b] = i {
                b > a
            } else {
                unreachable!()
            }
        })
        .count()
}

fn solve_b(input: &str) -> usize {
    let m: Vec<u32> = input.lines().map(str::parse).map(Result::unwrap).collect();
    let wins: Vec<u32> = m.windows(3).map(|w| w.iter().sum()).collect();
    wins.windows(2)
        .filter(|i| {
            if let [a, b] = i {
                b > a
            } else {
                unreachable!()
            }
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_2021_01_a() {
        assert_eq!(solve_a(&input()), 1475);
    }

    #[test]
    fn solution_2021_01_b() {
        assert_eq!(solve_b(&input()), 1516);
    }
}
