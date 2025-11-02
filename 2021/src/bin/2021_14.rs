// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/14

use std::collections::BTreeMap;

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/14.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let s: Vec<char> = input.lines().next().unwrap().chars().collect();
    let m = parse_map(input);

    // The thing is to work in counts of pairs. We don't care about the ordering of pairs.
    // Every insertion rule, on every step, produces two pairs, for which we increase the
    // counts.

    let mut pc: BTreeMap<[char; 2], usize> = BTreeMap::new(); // count of pairs
    for i in 0..(s.len() - 1) {
        *pc.entry([s[i], s[i + 1]]).or_default() += 1;
    }
    let mut sol_a = 0;
    for step in 1..=40 {
        // Each pair expands to two new pairs.
        let mut npc: BTreeMap<[char; 2], usize> = BTreeMap::new();
        for (pair, &n) in &pc {
            let insert = *m.get(pair).unwrap();
            *npc.entry([pair[0], insert]).or_default() += n;
            *npc.entry([insert, pair[1]]).or_default() += n;
        }
        pc = npc;
        if step == 10 {
            sol_a = difference(&s, &pc);
        }
    }
    let sol_b = difference(&s, &pc);
    (sol_a, sol_b)
}

/// Return the difference in frequency between the most common and least common
/// characters.
fn difference(s: &[char], pc: &BTreeMap<[char; 2], usize>) -> usize {
    let mut top = 0;
    let mut bot = usize::MAX;
    for c in 'A'..='Z' {
        // Since the pairs overlap, count this character where it occurs at the start
        // of each pair, plus as a special case the final character of the input,
        // which never changes.
        let mut tot = pc
            .iter()
            .filter(|(pair, _)| pair[0] == c)
            .map(|(_, n)| n)
            .sum();
        if c == s[s.len() - 1] {
            tot += 1;
        }
        if tot > 0 {
            // println!("{} {:30}", c, tot);
            top = std::cmp::max(top, tot);
            bot = std::cmp::min(bot, tot);
        }
    }
    top - bot
}

fn parse_map(input: &str) -> BTreeMap<[char; 2], char> {
    let mut from: BTreeMap<[char; 2], char> = BTreeMap::new();
    for l in input.lines().skip(2) {
        let k: Vec<char> = l.chars().take(2).collect();
        let k = [k[0], k[1]];
        let v = l.chars().nth(6).unwrap();
        from.insert(k, v);
    }
    from
}

#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn example() {
        assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 2194);
        assert_eq!(b, 2360298895777);
    }
}
