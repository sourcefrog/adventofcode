pub fn main() {
    println!("dec01 a: {}", solve_a());
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

fn load_input() -> Vec<isize> {
    std::fs::read_to_string("input/dec01.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}
