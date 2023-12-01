//! https://adventofcode.com/2022/day/2

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/02.txt").unwrap()
}

// enum Move {Rock, Paper, Scissors}
// enum Outcome {Win, Lose, Draw}

fn solve_a(input: &str) -> usize {
    let mut total = 0;
    for l in input.lines() {
        // let them = l.chars().nth(0).unwrap();
        let us = l.chars().nth(2).unwrap();
        match us {
            'X' => total += 1,
            'Y' => total += 2,
            'Z' => total += 3,
            _ => panic!("bad input {l:?}"),
        }
        total += match l {
            "A X" => 3,
            "A Y" => 6,
            "A Z" => 0,
            "B X" => 0,
            "B Y" => 3,
            "B Z" => 6,
            "C X" => 6,
            "C Y" => 0,
            "C Z" => 3,
            _ => panic!("unexpected {l:?}"),
        }
    }
    total
}

fn solve_b(input: &str) -> usize {
    let mut total = 0;
    for l in input.lines() {
        total += match l {
            "A X" =>
            /* lose to rock, play scissors */
            {
                3
            }
            "A Y" =>
            /* draw with rock, play rock */
            {
                1 + 3
            }
            "A Z" =>
            /* win against rock, play paper */
            {
                2 + 6
            }
            "B X" =>
            /* lose to paper, play rock */
            {
                1
            }
            "B Y" =>
            /* draw to paper, play paper */
            {
                2 + 3
            }
            "B Z" =>
            /* win against paper, play scissors */
            {
                3 + 6
            }
            "C X" =>
            /* lose to scissors, play paper */
            {
                2
            }
            "C Y" =>
            /* draw to scissors, play scissors */
            {
                3 + 3
            }
            "C Z" =>
            /* win against scissors, play rock */
            {
                1 + 6
            }
            _ => panic!("unexpected {l:?}"),
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9759);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 12429);
    }
}
