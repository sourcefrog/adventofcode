//! https://adventofcode.com/2022/day/20

use std::collections::HashMap;

use itertools::Itertools;

static EX: &str = "\
1
2
-3
3
-2
0
4
";

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/20.txt").unwrap()
}

fn solve_a(input: &str) -> isize {
    let n = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec();
    assert!(n.iter().all_unique());
    // OK let's treat it as a linked list where each number knows the following number.
    let mut next: HashMap<isize, isize> = HashMap::new();
    let mut prev: HashMap<isize, isize> = HashMap::new();
    for (&a, &b) in n.iter().circular_tuple_windows() {
        next.insert(a, b);
        prev.insert(b, a);
    }
    assert_eq!(next.len(), n.len());
    for i in n {
        // first, work out where we will insert it.
        let mut after: isize;
        let mut before: isize;
        if i == 0 {
            continue;
        } else if i > 0 {
            after = i;
            for _ in 0..i {
                after = *next.get(&after).unwrap();
            }
            before = *next.get(&after).unwrap();
        } else {
            before = i;
            for _ in 0..(-i) {
                before = prev[&before];
            }
            after = next[&before];
        }
        println!("move {i} to between {before} and {after}");
        let a = prev[&i];
        let b = next[&i];
        next.insert(a, b);
        prev.insert(b, a);
        next.insert(before, i);
        prev.insert(i, before);
        next.insert(i, after);
        prev.insert(after, i);

        for (&a, &b) in next.iter() {
            assert_eq!(prev[&b], a);
        }
    }

    // Not 13634 :(
    let mut prod = 1;
    let mut pos = 0;
    for _ in [1, 2, 3] {
        for _ in 0..1000 {
            pos = next[&pos];
        }
        prod *= pos;
    }
    prod
}

// fn solve_b(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_a(EX), 3);
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 9900);
    // }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);
    // }
}
