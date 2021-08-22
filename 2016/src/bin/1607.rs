//! https://adventofcode.com/2016/day/7

const DAY: &str = "1607";

fn solve_type_a(input: &str) -> usize {
    input.lines().filter(|l| supports_tls(l)).count()
}

/// Parse into a vec of components outside of brackets, and a
/// vec that are inside.
fn parse(l: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut outside = Vec::new();
    let mut inside = Vec::new();
    let mut rest = l.trim();
    while !rest.is_empty() {
        if let Some((a, b)) = rest.split_once('[') {
            outside.push(a.chars().collect());
            let (c, d) = b.split_once(']').expect("closing brace");
            inside.push(c.chars().collect());
            rest = d;
        } else {
            // No more brackets; the remainder is outside, and we're done
            outside.push(rest.chars().collect());
            break;
        }
    }
    (outside, inside)
}

fn supports_tls(l: &str) -> bool {
    let (outside, inside) = parse(l);
    outside.iter().any(|s| is_abba(s)) && !inside.iter().any(|s| is_abba(s))
}

fn is_abba(a: &[char]) -> bool {
    a.windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn solve_type_b(input: &str) -> usize {
    input.lines().filter(|l| supports_ssl(l)).count()
}

fn supports_ssl(l: &str) -> bool {
    let (supernets, hypernets) = parse(l);
    supernets
        .iter()
        .flat_map(|a| a.windows(3).filter(|w| is_aba(w)))
        .any(|aba| hypernets.iter().any(|hn| contains_bab(hn, aba)))
}

fn is_aba(a: &[char]) -> bool {
    a[0] == a[2] && a[0] != a[1]
}

fn contains_bab(hn: &[char], aba: &[char]) -> bool {
    debug_assert_eq!(aba.len(), 3);
    hn.windows(3)
        .any(|w| w[0] == aba[1] && w[1] == aba[0] && w[2] == aba[1])
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn solve_b() -> usize {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_a());
    println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 118);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 260);
    }
}
