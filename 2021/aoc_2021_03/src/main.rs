//! https://adventofcode.com/2021/day/3

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let n = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut ones = vec![0; cols];
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            if c == '1' {
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

fn count(a: &[&str], i: usize) -> (usize, usize) {
    let ones = a
        .iter()
        .filter(|l| l.chars().nth(i).unwrap() == '1')
        .count();
    (ones, a.len() - ones)
}

fn solve_b(input: &str) -> usize {
    let n = input.lines().count();

    let mut oxy_cands: Vec<&str> = input.lines().collect();
    for i in 0..n {
        let (ones, zeroes) = count(&oxy_cands, i);
        let crit = if ones >= zeroes { '1' } else { '0' };
        // println!("{}/{} ones so crit={}", ones, oxy_cands.len(), crit);
        oxy_cands.retain(|l| l.chars().nth(i).unwrap() == crit);
        // dbg!(oxy_cands.len());
        if oxy_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(oxy_cands.len(), 1);
    let oxy_rating = usize::from_str_radix(oxy_cands[0], 2).unwrap();

    let mut co2_cands: Vec<&str> = input.lines().collect();
    for i in 0..n {
        let (ones, zeroes) = count(&co2_cands, i);
        let crit = if ones < zeroes { '1' } else { '0' };
        println!("{}/{} ones so crit={}", ones, co2_cands.len(), crit);
        co2_cands.retain(|l| l.chars().nth(i).unwrap() == crit);
        if co2_cands.len() == 1 {
            break;
        }
    }
    assert_eq!(co2_cands.len(), 1);
    let co2_rating = usize::from_str_radix(co2_cands[0], 2).unwrap();
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
