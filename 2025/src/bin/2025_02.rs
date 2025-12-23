fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(',')
        .map(|x| {
            let t = x.split_once('-').expect("dash separator");
            (t.0.trim().parse().unwrap(), t.1.trim().parse().unwrap())
        })
        .collect()
}

fn is_repeated_decimal(x: usize) -> bool {
    // a bit of a cheesy way to work out if the digits repeat
    let s = x.to_string();
    let l = s.len();
    let b = s.as_bytes();
    b[0..(l / 2)] == b[(l / 2)..]
}

fn solve1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(|x| is_repeated_decimal(*x))
        .sum()
}

fn solve2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 1227775554);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 53420042388);
    }

    #[test]
    fn solution2() {
        // assert_eq!(solve2(&input()), 5978);
    }
}
