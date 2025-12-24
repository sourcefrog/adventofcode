use std::cmp::max;

fn main() {
    let input = input();
    println!("{}", solve1(&input,));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

type Point = [usize; 2];

fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .expect("2 elements")
        })
        .collect()
}

fn area(p1: &Point, p2: &Point) -> usize {
    (p1[0].abs_diff(p2[0]) + 1) * (p1[1].abs_diff(p2[1]) + 1)
}

fn solve1(input: &str) -> usize {
    let points: Vec<Point> = parse(input);
    let npoints = points.len();
    let mut best_area = 0;
    for i in 0..npoints {
        for j in (i + 1)..npoints {
            best_area = max(best_area, area(&points[i], &points[j]));
        }
    }
    best_area
}

fn solve2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
        "\
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE,), 50);
    }

    // #[test]
    // fn example2() {
    //     assert_eq!(solve2(EXAMPLE), 25272);
    // }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input(),), 4763509452);
    }

    // #[test]
    // fn solution2() {
    //     assert_eq!(solve2(&input()), 3767453340);
    // }
}
