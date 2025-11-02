use std::{fs::read_to_string, io::ErrorKind};

use aoclib::{Matrix, point};

fn main() {
    let input = &input();
    println!("{}", solve_one(input));
}

fn input() -> String {
    static PUZZLE: &str = env!("CARGO_BIN_NAME");
    let filename = format!("input/{PUZZLE}.txt");
    let mut path = filename;
    for _ in 0..5 {
        match read_to_string(&path) {
            Ok(s) => return s,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                path = "../".to_owned() + &path;
                continue;
            }
            Err(e) => panic!("{e:?}"),
        }
    }
    panic!("input not found in parents");
}

fn solve_one(input: &str) -> usize {
    let mut map = Matrix::from_string_lines(input);
    slide_north(&mut map);
    println!("{}", map.to_string_lines());
    calc_load(&map)
}

/// Slide all round rocks north as far as they will go, until things stop moving.
fn slide_north(map: &mut Matrix<char>) {
    // Work one column at a time: move round rocks as far north as they will go,
    // without moving square rocks.
    for col in 0..(map.width() as isize) {
        for r1 in 1..(map.height() as isize) {
            if map[point(col, r1)] == 'O' {
                let mut move_to = None;
                for r2 in (0..r1).rev() {
                    if map[point(col, r2)] == '.' {
                        move_to = Some(r2);
                    } else {
                        break;
                    }
                }
                if let Some(move_r) = move_to {
                    assert!(move_r < r1);
                    map[point(col, r1)] = '.';
                    map[point(col, move_r)] = 'O';
                }
            }
        }
    }
}

fn calc_load(map: &Matrix<char>) -> usize {
    let h = map.height();
    map.point_values()
        .filter(|(_p, v)| **v == 'O')
        .map(|(p, _v)| h - TryInto::<usize>::try_into(p.y).unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{input, solve_one};

    static EXAMPLE: &str = indoc! { "\
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#.... 
        "};

    #[test]
    fn example_one() {
        let sol = solve_one(EXAMPLE);
        assert_eq!(sol, 136);
    }

    #[test]
    fn solution_one() {
        assert_eq!(solve_one(&input()), 105003);
    }
}
