//! https://adventofcode.com/2022/day/12

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/12.txt").unwrap()
}

/// True if you can step from level x to level y.
fn can_move(mut x: &char, mut y: &char) -> bool {
    if *x == 'S' {
        x = &'a'
    };
    if *y == 'E' {
        y = &'z'
    };
    (*y as u16) <= (*x as u16 + 1)
}

fn solve_a(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let start = map.find_single_value(&'S');
    let end = map.find_single_value(&'E');
    map.shortest_path(start, end, can_move)
        .expect("no path")
        .distance()
}

fn solve_b(input: &str) -> usize {
    let map = Matrix::from_string_lines(input);
    let end = map.find_single_value(&'E');
    map.find_values(&'S')
        .chain(map.find_values(&'a'))
        .flat_map(|start| map.shortest_path(start, end, can_move))
        .map(|path| path.distance())
        .min()
        .expect("no paths found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 408);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 399);
    }
}
