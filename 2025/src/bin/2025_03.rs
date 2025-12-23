fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("digit") as u8)
                .collect()
        })
        .collect()
}

fn solve1(input: &str) -> usize {
    // for each line, find the earliest position of the maximum value in that line,
    // and then the maximum value after that
    parse(input)
        .into_iter()
        .map(|l| {
            // handle the case where the max value is at the end; we can't take that
            let max1 = *l[..(l.len() - 1)].iter().max().unwrap();
            let imax1 = l.iter().position(|x| *x == max1).unwrap();
            let max2 = *l[(imax1 + 1)..].iter().max().unwrap();
            (max1 * 10 + max2) as usize
        })
        .sum()
}

fn solve2(input: &str) -> usize {
    parse(input).iter().map(|l| best12(l)).sum()
}

fn best12(row: &[u8]) -> usize {
    let mut jolts = 0;
    let mut startpos = 0;
    for i in (0..12).rev() {
        let mut m: Option<u8> = None;
        let mut mpos = None;
        for (j, &x) in row.iter().enumerate().take(row.len() - i).skip(startpos) {
            if m.is_none_or(|m| x > m) {
                mpos = Some(j);
                m = Some(x);
            }
        }
        startpos = mpos.unwrap() + 1;
        jolts += m.unwrap() as usize;
        if i > 0 {
            jolts *= 10;
        }
    }
    jolts
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 357);
    }

    #[test]
    fn test_best12() {}

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 17158);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 170449335646486);
    }
}
