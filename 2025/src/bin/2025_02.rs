use std::collections::HashSet;

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

fn decimal_len(x: usize) -> usize {
    x.ilog10() as usize + 1
}

/// Input is a list of ranges.
///
/// Return the sum of the integers composed of twice-repeated runs of digits that
/// occur within any of the ranges.
fn solve1(input: &str) -> usize {
    let ranges = parse(input);
    let maxtop = ranges.iter().map(|a| a.1).max().unwrap().to_owned();
    let mut shift = 10;
    let mut sum = 0;
    let mut mult = 11; // contains two 1s in decimal forms with some zeros between; produces repeats of the right length
    let mut r = 0; // current value containing repeats
    for i in 1.. {
        if i >= shift {
            shift *= 10; // at 10, step from 99 to 1010; at 1000 we step from 999999 to 10001000, etc.
            mult = shift + 1;
            r = mult * i;
        } else {
            r += mult;
        }
        if r > maxtop {
            break;
        }
        for &(a, b) in &ranges {
            if (a..=b).contains(&r) {
                sum += r;
            }
        }
    }
    sum
}

fn solve2(input: &str) -> usize {
    let ranges = parse(input);
    let maxtop = ranges.iter().map(|a| a.1).max().unwrap().to_owned();
    let ranges = ranges.into_iter().map(|(a, b)| a..=b).collect::<Vec<_>>();
    let mut sum = 0;
    let mut hits = HashSet::new(); // numbers produced twice don't count twice
    for reps in 2..=decimal_len(maxtop) {
        // dbg!(reps);
        let mut shift = 1; // increases 1, 10, 100, etc, as the length of i increases
        let mut r = 0; // current value containing repeats
        let mut mult = 1; // contains `reps` decimal ones, spaced `shift` apart
        for i in 1.. {
            if i >= shift {
                assert_eq!(i, shift);
                shift *= 10; // at 10, step from 99 to 1010; at 1000 we step from 999999 to 10001000, etc.
                mult = (1..reps).fold(1, |m, _| m * shift + 1); // produce 1001001001001 etc
                r = mult * i;
            } else {
                r += mult;
            }
            if r > maxtop {
                break;
            }
            // dbg!(r);
            if ranges.iter().any(|range| range.contains(&r)) && hits.insert(r) {
                // println!("hit on {r}");
                sum += r;
            }
        }
    }
    // println!("total hits {}", hits.len());
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn simple() {
        assert_eq!(solve1("11-22"), 11 + 22);
    }

    #[test]
    fn simple2() {
        assert_eq!(solve2("83-225"), 88 + 99 + 111 + 222);
    }

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 1227775554);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 4174379265);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 53420042388);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 69553832684);
    }
}
