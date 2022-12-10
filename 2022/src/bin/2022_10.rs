//! https://adventofcode.com/2022/day/10

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/10.txt").unwrap()
}

fn solve_a(input: &str) -> isize {
    fn m(cycle: isize) -> bool {
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => true,
            _ => false,
        }
    }

    let mut x = 1;
    let mut cycle = 1isize;
    let mut tot = 0;
    for l in input.lines() {
        if m(cycle) {
            tot += cycle * x
        }
        cycle += 1;
        if let Some(v) = l.strip_prefix("addx ") {
            if m(cycle) {
                tot += cycle * x
            }
            x += v.parse::<isize>().unwrap();
            cycle += 1;
        }
    }
    tot
}

fn solve_b(input: &str) -> String {
    fn hit(x: isize, cycle: isize, display: &mut String) {
        if cycle > 240 {
            return;
        }
        if (x - ((cycle - 1) % 40)).abs() < 2 {
            display.push('#')
        } else {
            display.push('.')
        }
        if (cycle - 1) % 40 == 39 {
            display.push('\n');
        }
    }

    let mut x = 1;
    let mut cycle = 1isize;
    let mut dis = String::new();
    for l in input.lines() {
        hit(x, cycle, &mut dis);
        cycle += 1;
        if let Some(v) = l.strip_prefix("addx ") {
            hit(x, cycle, &mut dis);
            cycle += 1;
            x += v.parse::<isize>().unwrap();
        }
    }
    dis
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 14360);
    }

    #[test]
    fn solution_b() {
        assert_eq!(
            solve_b(&input()),
            "\
###...##..#..#..##..####.###..####.####.
#..#.#..#.#.#..#..#.#....#..#.#.......#.
###..#....##...#..#.###..#..#.###....#..
#..#.#.##.#.#..####.#....###..#.....#...
#..#.#..#.#.#..#..#.#....#.#..#....#....
###...###.#..#.#..#.####.#..#.####.####.
"
        );
    }
}
