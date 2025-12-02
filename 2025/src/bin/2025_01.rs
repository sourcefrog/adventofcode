fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve1(input: &str) -> usize {
    const SIZE: isize = 100;
    let mut p: isize = 50;
    let mut zeros = 0;
    for l in input.trim().lines() {
        let dir = match l.chars().next().unwrap() {
            'R' => 1,
            'L' => -1,
            _ => panic!(),
        };
        let dist: isize = l[1..].parse().unwrap();
        let mut new = (p + (dir * dist)) % SIZE;
        if new < 0 {
            new += SIZE;
        }
        assert!((0..SIZE).contains(&new));
        if new == 0 {
            zeros += 1;
        }
        p = new;
    }
    zeros
}

fn solve2(input: &str) -> usize {
    const SIZE: isize = 100;
    let mut p: isize = 50;
    let mut clicks: usize = 0;
    for l in input.trim().lines().map(str::trim) {
        let dist: isize = l[1..].parse().unwrap();
        match l.chars().next().unwrap() {
            'R' => {
                clicks += usize::try_from((p + dist) / SIZE).unwrap();
                p = (p + dist) % SIZE;
            }
            'L' => {
                let q = p - dist;
                if q <= 0 {
                    clicks += usize::try_from(-q / SIZE).unwrap();
                    if q % SIZE == 0 {
                        clicks += 1
                    }
                }
                p = q % SIZE;
                if p < 0 {
                    p += SIZE
                }
                assert!((0..SIZE).contains(&p));
            }
            _ => panic!(),
        };
        println!("{l}\t{clicks}\t{p}");
    }
    // 6499 is too high
    clicks
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    ";

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 997);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 6);
    }
}
