use aoclib::{Matrix, Point};

fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn parse(input: &str) -> Matrix<bool> {
    Matrix::from_string_lines(input).map(|c| *c == '@')
}

fn solve1(input: &str) -> usize {
    let mat = parse(input);
    mat.points()
        .filter(|&p| mat[p] && mat.neighbors8(p).filter(|(_p, c)| **c).count() < 4)
        .count()
}

fn solve2(input: &str) -> usize {
    let mut tot = 0;
    let mut mat = parse(input);
    loop {
        let can_move = mat
            .points()
            .filter(|&p| mat[p] && mat.neighbors8(p).filter(|(_p, c)| **c).count() < 4)
            .collect::<Vec<Point>>();
        if can_move.is_empty() {
            break;
        }
        tot += can_move.len();
        can_move.iter().for_each(|&p| mat[p] = false);
    }
    tot
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
        "..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@."
    };

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 43);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 1411);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 8557);
    }
}
