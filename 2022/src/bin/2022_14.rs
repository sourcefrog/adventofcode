//! https://adventofcode.com/2022/day/14

use aoclib::{line_between, point, Matrix, Point};

static TAP: Point = point(500, 0);

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/14.txt").unwrap()
}

fn load(input: &str) -> Matrix<char> {
    let draw: Vec<Vec<Point>> = input
        .lines()
        .map(|l| l.split(" -> ").map(|xy| xy.parse().unwrap()).collect())
        .collect();
    let mut mat = Matrix::new(1200, 600, '.'); // empirical/guessed size
    for shape in draw {
        for pair in shape.windows(2) {
            for pq in line_between(pair[0], pair[1]) {
                mat[pq] = '#'
            }
        }
    }
    mat
}

fn solve_a(input: &str) -> usize {
    let mut mat = load(input);
    's: while mat[TAP] == '.' {
        let mut sp = TAP;
        loop {
            if (sp.y + 1) >= mat.height() as isize {
                // fell off the bottom
                break 's;
            } else if mat[sp.down()] == '.' {
                sp = sp.down();
            } else if mat[sp.down().left()] == '.' {
                sp = sp.down().left();
            } else if mat[sp.down().right()] == '.' {
                sp = sp.down().right();
            } else {
                // can't move
                assert_eq!(mat[sp], '.');
                println!("fill {sp:?}");
                mat[sp] = 'o';
                continue 's;
            }
        }
    }
    mat.find_values(&'o').count()
}

fn solve_b(input: &str) -> usize {
    let mut mat = load(input);
    let bottom = mat.find_values(&'#').map(|p| p.y).max().unwrap() + 1;
    's: while mat[TAP] == '.' {
        let mut sp = TAP;
        loop {
            if sp.y == bottom {
                assert_eq!(mat[sp], '.');
                // println!("bottom at {sp:?}");
                mat[sp] = 'o';
                continue 's;
            } else if mat[sp.down()] == '.' {
                sp = sp.down();
            } else if mat[sp.down().left()] == '.' {
                sp = sp.down().left();
            } else if mat[sp.down().right()] == '.' {
                sp = sp.down().right();
            } else {
                // can't move
                assert_eq!(mat[sp], '.');
                // println!("fill {sp:?}");
                mat[sp] = 'o';
                continue 's;
            }
        }
    }
    mat.find_values(&'o').count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1133);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 27566);
    }
}
