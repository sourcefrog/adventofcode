//! https://adventofcode.com/2021/day/3

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn to_matrix(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '1').collect())
        .collect()
}

fn solve_a(input: &str) -> usize {
    let m = to_matrix(input);
    let n = m.len();
    let cols = m[0].len();
    let mut ones = vec![0; cols];
    for l in m {
        for (i, c) in l.iter().enumerate() {
            if *c {
                ones[i] += 1
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for a in ones {
        gamma <<= 1;
        epsilon <<= 1;
        if a > (n - a) {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    gamma * epsilon
}

fn count(a: &[&[bool]], i: usize) -> (usize, usize) {
    let ones = a.iter().filter(|l| l[i]).count();
    (ones, a.len() - ones)
}

fn from_base2(a: &[bool]) -> usize {
    let mut x = 0;
    for c in a {
        x = (x << 1) | (*c as usize);
    }
    x
}

fn solve_b(input: &str) -> usize {
    let m = to_matrix(input);
    let n = m.len();

    let mut oxy_cands: Vec<&[bool]> = m.iter().map(|l| l.as_ref()).collect();
    let mut co2_cands: Vec<&[bool]> = oxy_cands.clone();
    for i in 0..n {
        let (ones, zeroes) = count(&oxy_cands, i);
        let crit = ones >= zeroes;
        oxy_cands.retain(|l| l[i] == crit);
        if oxy_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(oxy_cands.len(), 1);
    let oxy_rating = from_base2(oxy_cands[0]);

    for i in 0..n {
        let (ones, zeroes) = count(&co2_cands, i);
        let crit = ones < zeroes;
        co2_cands.retain(|l| l[i] == crit);
        if co2_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(co2_cands.len(), 1);
    let co2_rating = from_base2(&co2_cands[0]);
    oxy_rating * co2_rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 4191876);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 3414905);
    }
}
