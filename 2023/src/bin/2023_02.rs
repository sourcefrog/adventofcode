use std::fs::read_to_string;

fn main() {
    println!("2023_01 a {}", solve_a(&input()));
    println!("2023_01 b {}", solve_b(&input()));
}

fn solve_a(input: &str) -> usize {
    let mut x = 0;
    'lines: for (i, line) in input.lines().enumerate() {
        let game_no = i + 1;
        let rest = line
            .strip_prefix(&format!("Game {}: ", game_no))
            .expect("find prefix");
        for draw in rest.split("; ") {
            dbg!(&draw);
            for ccount in draw.split(", ") {
                let (n, color) = ccount.split_once(' ').unwrap();
                let n: usize = n.parse().expect("parse n");
                let color_ok = match color {
                    "red" => n <= 12,
                    "green" => n <= 13,
                    "blue" => n <= 14,
                    _ => panic!("{color:?} in {line:?}"),
                };
                if !color_ok {
                    continue 'lines;
                }
            }
        }
        x += game_no;
    }
    x
}

fn solve_b(input: &str) -> usize {
    let mut x = 0;
    for (i, line) in input.lines().enumerate() {
        let game_no = i + 1;
        let rest = line
            .strip_prefix(&format!("Game {}: ", game_no))
            .expect("find prefix");
        let mut game_min = [0, 0, 0];
        for draw in rest.split("; ") {
            dbg!(&draw);
            for ccount in draw.split(", ") {
                let (n, color) = ccount.split_once(' ').unwrap();
                let n: usize = n.parse().expect("parse n");
                let idx = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("{color:?} in {line:?}"),
                };
                game_min[idx] = std::cmp::max(game_min[idx], n);
            }
        }
        x += game_min.iter().product::<usize>();
    }
    x
}

fn input() -> String {
    read_to_string("input/02.txt").unwrap()
}

struct Rgb {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(s: &str) -> Vec<Vec<Rgb>> {
    let mut games = Vec::new();
    for (i, line) in s.lines().enumerate() {
        let mut draws = Vec::new();
        let rest = line
            .strip_prefix(&format!("Game {}: ", i + 1))
            .expect("find prefix");
        games.push(draws);
    }
    games
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn solution_a() {}

    // b: 54623 is too high.

    #[test]
    fn example_1() {}
}
