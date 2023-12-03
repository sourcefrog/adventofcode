use std::collections::{BTreeMap, BTreeSet};
use std::fs::read_to_string;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = &input();
    println!("{}", solve_b(input))
}

fn input() -> String {
    read_to_string("2023/input/03.txt")
        .or_else(|_| read_to_string("input/03.txt"))
        .unwrap()
}

fn solve_a(input: &str) -> usize {
    let mat = Matrix::from_string_lines(input);
    let mut tot = 0;
    for row in 0..mat.height() {
        let mut num = String::new();
        let mut near_sym = false;
        for col in 0..mat.width() {
            let p = point(col as isize, row as isize);
            let c = mat[p];
            if c.is_ascii_digit() {
                num.push(c);
                for (_np, nc) in mat.neighbors8(p) {
                    if !nc.is_ascii_digit() && *nc != '.' {
                        near_sym = true;
                        break;
                    }
                }
            } else if !num.is_empty() {
                if near_sym {
                    tot += num.parse::<usize>().expect("num is decimal");
                    near_sym = false;
                }
                num.clear();
            }
        }
        if !num.is_empty() && near_sym {
            tot += num.parse::<usize>().expect("num is decimal");
        }
    }
    tot
}

fn solve_b(input: &str) -> usize {
    let mat = Matrix::from_string_lines(input);
    let mut gears: BTreeMap<Point, Vec<usize>> = BTreeMap::new();
    for row in 0..mat.height() {
        let mut num = String::new();
        let mut near_gears: BTreeSet<Point> = Default::default();
        for col in 0..mat.width() {
            let p = point(col as isize, row as isize);
            let c = mat[p];
            if c.is_ascii_digit() {
                num.push(c);
                for (np, nc) in mat.neighbors8(p) {
                    if *nc == '*' {
                        near_gears.insert(np);
                    }
                }
            } else if !num.is_empty() {
                let val = num.parse::<usize>().expect("num is decimal");
                for g in &near_gears {
                    gears.entry(*g).or_default().push(val);
                }
                num.clear();
                near_gears.clear();
            }
        }
        if !num.is_empty() {
            let val = num.parse::<usize>().expect("num is decimal");
            for g in &near_gears {
                gears.entry(*g).or_default().push(val);
            }
        }
    }
    gears
        .values()
        .filter(|vs| vs.len() == 2)
        .map(|vs| vs.iter().product::<usize>())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(solve_a(input), 4361);
    }

    #[test]
    fn example_2() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(solve_b(input), 467835);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 556367);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 89471771);
    }
}
