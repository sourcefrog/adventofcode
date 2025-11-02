use std::fs::read_to_string;

static PUZZLE: &str = env!("CARGO_BIN_NAME");

fn main() {
    let input = &input();
    println!("{PUZZLE} a {}", solve_a(input));
    println!("{PUZZLE} b {}", solve_b(input));
}

fn input() -> String {
    let (year, day) = PUZZLE.split_once('_').unwrap();
    read_to_string(format!("{year}/input/{day}.txt"))
        .or_else(|_| read_to_string(format!("input/{day}.txt")))
        .unwrap()
}

fn solve_a(input: &str) -> usize {
    /* We can definitely proceed one line at a time and then sum them up.
     *
     */
    todo!()
}

fn solve_b(input: &str) -> usize {
    let _ = input;
    0
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"\
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    " };

    #[test]
    #[ignore = "not working yet"]
    fn example_1() {
        assert_eq!(solve_a(EXAMPLE), 21);
    }

    #[test]
    fn solution_a() {
        // assert_eq!(solve_a(&input()), 24706);
    }

    #[test]
    fn solution_b() {
        // assert_eq!(solve_b(&input()), 13114317);
    }
}
