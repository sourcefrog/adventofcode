//! https://adventofcode.com/2022/day/7

use std::collections::BTreeMap;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/07.txt").unwrap()
}

fn solve_a(input: &str) -> usize {
    let mut cwd: Vec<&str> = vec!["/"];
    let mut dir_size: BTreeMap<Vec<&str>, usize> = BTreeMap::new();
    for l in input.lines() {
        if l == "$ cd /" {
        } else if l.starts_with("$ cd ") {
            let new = l.strip_prefix("$ cd ").unwrap();
            if new == ".." {
                cwd.pop().unwrap();
            } else {
                cwd.push(new);
            }
        } else if l == "$ ls" {
            assert!(dir_size.insert(cwd.clone(), 0).is_none());
        } else if l.starts_with("$ ") {
            panic!("unhandled {l:?}")
        } else if l.starts_with("dir") {
            // nothing for now
        } else {
            let (size, _name) = l.split_once(" ").unwrap();
            let size = size.parse::<usize>().unwrap();
            // Add to every enclosing parent.
            let mut p = cwd.clone();
            while !p.is_empty() {
                *dir_size.get_mut(&p).unwrap() += size;
                p.pop();
            }
        }
    }
    // dir_size.values().filter(|s| **s <= 100000).sum::<usize>()
    let total_space = 70000000;
    let needed_space = 30000000;
    let need_del = dir_size.get(&vec!["/"]).unwrap() - (total_space - needed_space);
    println!("need to delete {need_del}");
    dir_size
        .values()
        .filter(|s| **s >= need_del)
        .cloned()
        .min()
        .unwrap()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 1034);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 2472);
    }
}
