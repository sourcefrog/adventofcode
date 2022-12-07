//! https://adventofcode.com/2022/day/7

use std::collections::BTreeMap;

fn main() {
    let (a, b) = solve(&input());
    println!("{a}\n{b}");
}

fn input() -> String {
    std::fs::read_to_string("input/07.txt").unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let mut cwd: Vec<&str> = vec!["/"];
    let mut dir_size: BTreeMap<Vec<&str>, usize> = BTreeMap::new();
    for l in input.lines() {
        if let Some(dir) = l.strip_prefix("$ cd ") {
            if dir == ".." {
                cwd.pop().unwrap();
            } else if dir == "/" {
                // never happens after the start
            } else {
                cwd.push(dir);
            }
        } else if l == "$ ls" {
            assert!(dir_size.insert(cwd.clone(), 0).is_none());
        } else if l.starts_with("$ ") {
            panic!("unhandled {l:?}")
        } else if l.starts_with("dir") {
            // nothing for now
        } else {
            let size = l.split(' ').next().unwrap().parse::<usize>().unwrap();
            // Add to every enclosing parent.
            let mut p = cwd.clone();
            while !p.is_empty() {
                *dir_size.get_mut(&p).unwrap() += size;
                p.pop();
            }
        }
    }
    let part1 = dir_size.values().filter(|s| **s <= 100000).sum::<usize>();

    let total_space = 70000000;
    let needed_space = 30000000;
    let need_del = dir_size.get(&vec!["/"]).unwrap() - (total_space - needed_space);
    let part2 = dir_size
        .values()
        .filter(|s| **s >= need_del)
        .cloned()
        .min()
        .unwrap();
    (part1, part2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        assert_eq!(solve(&input()), (1501149, 10096985));
    }
}
