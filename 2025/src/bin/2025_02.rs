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

/// True if the decimal form of `x` contains `repeats` copies of its first digits.
fn is_repeated_decimal(x: usize, repeats: usize) -> bool {
    assert!(repeats >= 2);
    if x == 0 {
        return false;
    }
    let digits = decimal_len(x);
    if !digits.is_multiple_of(repeats) {
        return false;
    }
    let s = 10usize.pow((digits / repeats) as u32);
    let v = x % s;
    x == repeat(v, repeats)
}

/// True if repeated with any length
fn is_repeat(x: usize) -> bool {
    assert!(x > 0);
    let digits = decimal_len(x);
    let mut s = 1;
    for d in 1..=(digits / 2) {
        s *= 10;
        if !digits.is_multiple_of(d) {
            continue;
        }
        let r = digits / d;
        let v = x % s;
        let y = (0..r).map(|j| s.pow(j as u32) * v).sum();
        if x == y {
            return true;
        }
    }
    false
}

/// Return the number that has `r` repeats of the digits in x.
fn repeat(x: usize, r: usize) -> usize {
    debug_assert!(r >= 2);
    if x == 0 {
        return 0;
    }
    let d = decimal_len(x);
    let s = 10usize.pow(d as u32);
    let mut a = x;
    for i in 1..r {
        a += s.pow(i as u32) * x;
    }
    a
}

fn solve1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(|x| is_repeated_decimal(*x, 2))
        .sum()
}

fn solve2(input: &str) -> usize {
    let input = parse(input);
    let mut s = 0;
    for (a, b) in input {
        // We don't really need to check all of them: if we're at
        // 12350000 then we really ought to be able to see that the next
        // useful number to check is 12351235 (at least for 2 repeats).
        // Although, it might get a bit complicated because we need
        // to be careful not to double count e.g. 333333 as being a repeat
        // of 3, 33 and 333 ...
        'i: for i in a..=b {
            if is_repeat(i) {
                s += i;
                continue 'i;
            }
        }
    }
    s
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

    #[test]
    fn test_is_repeat() {
        assert!(is_repeat(11));
        assert!(is_repeat(1010));
    }
}
