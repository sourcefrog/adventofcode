//! https://adventofcode.com/2022/day/8

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/08.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mat = aoclib::Matrix::from_string_lines(input);
    let mat = mat.map(|c| c.to_digit(10).unwrap() as isize);
    let mut vis = Matrix::same_size(&mat, false);
    for row in 0..mat.height() {
        let mut h = -1;
        for p in mat.row_points(row) {
            let c = mat[p];
            if c > h {
                vis[p] = true;
                h = c;
            }
        }
        let mut h = -1;
        for p in mat.row_points(row).rev() {
            let c = mat[p];
            if c > h {
                vis[p] = true;
                h = c;
            }
        }
    }
    for col in 0..mat.width() {
        let mut h = -1;
        for p in mat.column_points(col) {
            let c = mat[p];
            if c > h {
                vis[p] = true;
                h = c;
            }
        }
        let mut h = -1;
        for p in mat.column_points(col).rev() {
            let c = mat[p];
            if c > h {
                vis[p] = true;
                h = c;
            }
        }
    }
    // println!("{}", vis.to_string_lines());
    vis.values().filter(|x| **x).count()
}

fn solve_b(input: &str) -> usize {
    let mat = aoclib::Matrix::from_string_lines(input);
    let mat = mat.map(|c| c.to_digit(10).unwrap());
    let mut best = 0;
    for (p, &c) in mat.point_values() {
        // up
        let mut ups = 0;
        for q in mat.points_up(p) {
            ups += 1;
            if mat[q] >= c {
                break;
            }
        }
        let mut downs = 0;
        for q in mat.points_down(p) {
            downs += 1;
            if mat[q] >= c {
                break;
            }
        }

        let mut lefts = 0;
        for q in mat.points_left(p) {
            lefts += 1;
            if mat[q] >= c {
                break;
            }
        }
        let mut rights = 0;
        for q in mat.points_right(p) {
            rights += 1;
            if mat[q] >= c {
                break;
            }
        }
        best = std::cmp::max(best, ups * downs * lefts * rights);
    }
    best
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1681);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 201684);
    }
}
