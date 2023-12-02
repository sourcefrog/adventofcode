use std::fs::read_to_string;

use regex::Regex;

fn main() {
    println!("2023_01 a {}", solve_a(&input()));
    println!("2023_01 b {}", solve_b(&input()));
}

fn solve_a(input: &str) -> usize {
    let mut x = 0;
    'game: for (game_no, game) in parse(input)
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
    {
        for (n, color) in game {
            if !match color {
                "red" => n <= 12,
                "green" => n <= 13,
                "blue" => n <= 14,
                _ => panic!("{color:?}"),
            } {
                continue 'game;
            }
        }
        x += game_no;
    }
    x
}

fn solve_b(input: &str) -> usize {
    let mut x = 0;
    for game in parse(input) {
        let mut game_min = [0, 0, 0];
        for (n, color) in game {
            let idx = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("{color:?}"),
            };
            game_min[idx] = std::cmp::max(game_min[idx], n);
        }
        x += game_min.iter().product::<usize>();
    }
    x
}

/// For each game (input line) return a vec of (count, color), regardless of rounds
fn parse(input: &str) -> Vec<Vec<(usize, &str)>> {
    let mut v = Vec::new();
    let re = Regex::new(r"[;,] ").expect("Compile RE");
    for (i, line) in input.lines().enumerate() {
        let game_no = i + 1;
        let mut game = Vec::new();
        let rest = line
            .strip_prefix(&format!("Game {}: ", game_no))
            .expect("find prefix");
        // The games don't actually matter; we only want to know each draw
        for draw in re.split(rest) {
            let (n, color) = draw.split_once(' ').unwrap();
            let n: usize = n.parse().expect("parse n");
            game.push((n, color));
        }
        v.push(game);
    }
    v
}

fn input() -> String {
    read_to_string("input/02.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 2369);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 66363);
    }
}
