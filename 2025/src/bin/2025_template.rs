fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve1(input: &str) -> usize {
    let _ = input;
    0
}

fn solve2(input: &str) -> usize {
    let _ = input;
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
                                                "\
    "};

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 0);
    }

    // #[test]
    // fn solution1() {
    //     assert_eq!(solve1(&input()), 0);
    // }

    // #[test]
    // fn solution2() {
    //     assert_eq!(solve2(&input()), 0);
    // }
}
