//! https://adventofcode.com/2022/day/21

use std::collections::HashMap;

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/21.txt").unwrap()
}

#[derive(Debug)]
enum Mk<'a> {
    Const(isize),
    Op(char, &'a str, &'a str),
}

fn solve_a(input: &str) -> isize {
    let mut mks: HashMap<&str, Mk> = HashMap::new();
    // List of dependencies: when k is available, everything in v _might_ be ready to be solved.
    let mut kids: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut qrdy: Vec<&str> = Vec::new();
    let mut vals: HashMap<&str, isize> = HashMap::new();
    for l in input.lines() {
        let (name, rest) = l.split_once(": ").unwrap();
        let m;
        if let Ok(v) = rest.parse::<isize>() {
            m = Mk::Const(v);
            qrdy.push(name);
        } else {
            let aop = l.split_ascii_whitespace().nth(1).unwrap();
            let bop = l.split_ascii_whitespace().nth(3).unwrap();
            let opch = l.chars().nth(11).unwrap();
            m = Mk::Op(opch, aop, bop);
            kids.entry(aop).or_default().push(name);
            kids.entry(bop).or_default().push(name);
        }
        assert!(mks.insert(name, m).is_none());
    }
    while let Some(n) = qrdy.pop() {
        let mk = &mks[n];
        println!("eval {n} {mk:?}");
        let v = match mk {
            Mk::Const(x) => *x,
            Mk::Op(opch, an, bn) => {
                let a = vals[an];
                let b = vals[bn];
                match opch {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!(),
                }
            }
        };
        println!("  => {v}");
        if n == "root" {
            return v;
        }
        vals.insert(n, v);
        // look,crudely, for any monkeys whose inputs are now available
        for n2 in &kids[n] {
            if !vals.contains_key(n2) {
                if let Mk::Op(_, aop, bop) = &mks[n2] {
                    if vals.contains_key(aop) && vals.contains_key(bop) {
                        qrdy.push(n2);
                    }
                }
            }
        }
    }
    unreachable!()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

static EX: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
