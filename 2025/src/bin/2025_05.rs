fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

struct Input {
    fresh: Vec<(usize, usize)>,
    avail: Vec<usize>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let mut fresh = Vec::new();
    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }
        let (a, b) = l.split_once('-').unwrap();
        fresh.push((a.parse().unwrap(), b.parse().unwrap()));
    }
    let avail = lines.map(|l| l.trim().parse().unwrap()).collect();
    Input { fresh, avail }
}

fn solve1(input: &str) -> usize {
    let input = parse(input);
    input
        .avail
        .iter()
        .filter(|&&a| input.fresh.iter().any(|&f| f.0 <= a && a <= f.1))
        .count()
}

fn solve2(input: &str) -> usize {
    let Input { mut fresh, .. } = parse(input);
    fresh.sort();
    // Proceed through the ranges keeping track of the number of fresh items that
    // we've seen so far...
    let mut pos = 0;
    let mut tot = 0;
    for (start, end) in fresh {
        if pos > end {
        } else if pos < start {
            tot += end + 1 - start;
            pos = end + 1;
        } else {
            tot += end + 1 - pos;
            pos = end + 1;
        }
    }
    tot
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
        "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
        "
    };

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 14);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 735);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 344306344403172);
    }
}
