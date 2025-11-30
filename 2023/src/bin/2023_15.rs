use aoclib::input;

fn main() {
    let input = &input();
    println!("{}", solve_one(input));
    println!("{}", solve_two(input));
}

fn input() -> String {
    input!()
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |h, c| ((h + *c as usize) * 17) & 0xff)
}

fn solve_one(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn solve_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn solution_1() {
        assert_eq!(solve_one(&input()), 517551);
    }
}
