//! https://adventofcode.com/2022/day/10

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/10.txt").unwrap()
}

// Return a list of `cycle, x` values in each successive cycle.
fn exec(input: &str) -> impl Iterator<Item = (isize, isize)> {
    let mut x = 1;
    let mut xs = Vec::new();
    for l in input.lines() {
        xs.push(x);
        if let Some(v) = l.strip_prefix("addx ") {
            xs.push(x);
            x += v.parse::<isize>().unwrap();
        }
    }
    (1..).zip(xs)
}

fn solve_a(input: &str) -> isize {
    exec(input)
        .filter(|(c, _x)| matches!(c, 20 | 60 | 100 | 140 | 180 | 220))
        .map(|(c, x)| c * x)
        .sum()
}

fn solve_b(input: &str) -> String {
    let mut dis = String::new();
    for (c, x) in exec(input).take(240) {
        let col = (c - 1) % 40;
        if (x - col).abs() < 2 {
            dis.push('#');
        } else {
            dis.push('.');
        }
        if col == 39 {
            dis.push('\n');
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
