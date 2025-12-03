const SIZE: isize = 100;

fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve1(input: &str) -> usize {
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
    let mut p: isize = 50;
    let mut clicks: isize = 0;
    for l in input.trim().lines().map(str::trim) {
        let dist: isize = l[1..].parse().unwrap();
        match l.chars().next().unwrap() {
            'R' => {
                clicks += (p + dist) / SIZE;
                p = (p + dist) % SIZE;
            }
            'L' => {
                let q = p - dist;
                clicks += clicks_left(p, dist);
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
    clicks.try_into().unwrap()
}

/// If you start at position `p` and move left by `dist` how many times do
/// we pass 0, including ending on zero?
fn clicks_left(p: isize, dist: isize) -> isize {
    assert!((0..SIZE).contains(&p));
    let mut q = p - dist;
    let mut clicks = 0;
    if p == 0 {
        clicks -= 1;
    }
    while q <= 0 {
        clicks += 1;
        q += SIZE;
    }
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
    fn left_examples() {
        assert_eq!(clicks_left(50, 68), 1);
        assert_eq!(clicks_left(82, 30), 0);
        assert_eq!(clicks_left(0, 5), 0);
        assert_eq!(clicks_left(55, 55), 1);
        assert_eq!(clicks_left(0, 1), 0);
        assert_eq!(clicks_left(99, 99), 1);
        assert_eq!(clicks_left(14, 82), 1);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 997);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 5978);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 6);
    }
}
