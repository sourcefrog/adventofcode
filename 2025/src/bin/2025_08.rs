use itertools::Itertools;

fn main() {
    let input = input();
    println!("{}", solve1(&input, 1000));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

type Point = [usize; 3];

/// Square of the Pythagorean distance between points
fn dist(p1: Point, p2: Point) -> usize {
    p1.iter()
        .zip(p2.iter())
        .map(|(&a, &b)| {
            let x = a.abs_diff(b);
            x * x
        })
        .sum()
}

fn solve1(input: &str, rounds: usize) -> usize {
    let points: Vec<Point> = input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .expect("3 elements")
        })
        .collect();
    // map from point number to circuit number: initially each point is in a distinct circuit
    let mut circuits = (0..points.len()).collect::<Vec<usize>>();
    // make a list of distances between distinct pairs of points
    let mut dists = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            dists.push((dist(points[i], points[j]), i, j));
        }
    }
    dists.sort();
    for (_distance, p1, p2) in dists.into_iter().take(rounds) {
        // println!("closest {i}: {:?}, {:?}", points[p1], points[p2]);
        let cct1 = circuits[p1];
        let cct2 = circuits[p2];
        if cct1 != cct2 {
            // println!("  join circuits");
            circuits.iter_mut().for_each(|c| {
                if *c == cct2 {
                    *c = cct1
                }
            });
        }
    }
    let mut circuitsizes = vec![0; points.len()];
    for cct in circuits {
        circuitsizes[cct] += 1;
    }
    circuitsizes.iter().sorted().rev().take(3).product()
}

fn solve2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
        "\
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE, 10), 40);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 0);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input(), 1000), 67488);
    }

    // #[test]
    // fn solution2() {
    //     assert_eq!(solve2(&input()), 0);
    // }
}
