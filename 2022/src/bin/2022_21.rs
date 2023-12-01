//! https://adventofcode.com/2022/day/21

use std::collections::HashMap;

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/21.txt").unwrap()
}

#[derive(Debug)]
enum Mk<'a> {
    Const(isize),
    Op(char, &'a str, &'a str),
}

impl<'a> Mk<'a> {
    fn eval(&self, vals: &HashMap<&'a str, isize>) -> isize {
        match self {
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
        }
    }
}
type Mkmap<'a> = HashMap<&'a str, Mk<'a>>;

/// Evaluate a monkey if it can be done without relying on humn, for part 2.
fn eval_maybe<'a>(
    name: &'a str,
    mks: &HashMap<&'a str, Mk<'a>>,
    memo: &mut HashMap<&'a str, isize>,
) -> Option<isize> {
    let mk = &mks[name];
    println!("attempt eval {name} {mk:?}");
    if name == "humn" {
        return None;
    }
    assert_ne!(name, "root");
    let v = match mk {
        Mk::Const(x) => Some(*x),
        Mk::Op(opch, an, bn) => {
            if let (Some(a), Some(b)) = (eval_maybe(an, mks, memo), eval_maybe(bn, mks, memo)) {
                Some(match opch {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!(),
                })
            } else {
                println!("evaluation of {name} stalled");
                None
            }
        }
    };
    if let Some(v) = v {
        memo.insert(name, v);
    }
    println!("result {name} => {v:?}");
    v
}

fn from_root(mks: &Mkmap) -> isize {
    let mk = &mks["root"];
    // one side will have a known value; one unknown
    let unn: &str;
    let known: isize;
    let mut memo: HashMap<&str, isize> = HashMap::new();
    match mk {
        Mk::Op(_, an, bn) => {
            let aval = eval_maybe(an, mks, &mut memo);
            let bval = eval_maybe(bn, mks, &mut memo);
            match (aval, bval) {
                (Some(a), None) => {
                    known = a;
                    unn = bn;
                }
                (None, Some(b)) => {
                    known = b;
                    unn = an;
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
    println!("root known value {known}, unknown side {unn}");
    push_down(unn, mks, known, &mut memo)
}

/// Given the known value of an expression push down towards the unknown human value and
/// eventually return it.
///
/// This should only be called for unknown values.
fn push_down(name: &str, mks: &Mkmap, val: isize, memo: &mut HashMap<&str, isize>) -> isize {
    if name == "humn" {
        println!("found humn {val}");
        return val;
    }
    let mk = &mks[name];
    println!("push down {name} = {val} into {mk:?}");
    match mk {
        // TODO: This could be combined with the similar code above in
        // from_root, although at the root nothing will be memoized
        // yet.
        Mk::Const(_) => panic!(),
        Mk::Op(opch, an, bn) => {
            let aval = memo.get(an);
            let bval = memo.get(bn);
            println!("a: {an}={aval:?}, b: {bn}={bval:?}");
            match (aval, bval) {
                (Some(a), None) => {
                    let dn = match opch {
                        '+' => val - a, // val = a + b; b = val - a
                        '-' => a - val, // val = a - b; b = a - val
                        '*' => val / a, // val = a * b; b = val / a
                        '/' => val / a, // val = a / b; b = a / val
                        _ => panic!(),
                    };
                    push_down(bn, mks, dn, memo)
                }
                (None, Some(b)) => {
                    let dn = match opch {
                        '+' => val - b, // val = a + b; a = val - b
                        '-' => b + val, // val = a - b; a = val + b
                        '*' => val / b, // val = a * b; a = val / b
                        '/' => val * b, // val = a / b; a = val * b
                        _ => panic!(),
                    };
                    push_down(an, mks, dn, memo)
                }
                _ => panic!(),
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Mk> {
    let mut mks: HashMap<&str, Mk> = HashMap::new();
    for l in input.lines() {
        let (name, rest) = l.split_once(": ").unwrap();
        let m;
        if let Ok(v) = rest.parse::<isize>() {
            m = Mk::Const(v);
        } else {
            let aop = l.split_ascii_whitespace().nth(1).unwrap();
            let bop = l.split_ascii_whitespace().nth(3).unwrap();
            let opch = l.chars().nth(11).unwrap();
            m = Mk::Op(opch, aop, bop);
        }
        assert!(mks.insert(name, m).is_none());
    }
    mks
}

fn solve_a(input: &str) -> isize {
    // List of dependencies: when k is available, everything in v _might_ be ready to be solved.
    let mks = parse(input);
    let mut kids: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut qrdy: Vec<&str> = Vec::new();
    let mut vals: HashMap<&str, isize> = HashMap::new();
    for (name, mk) in &mks {
        match mk {
            Mk::Const(_) => qrdy.push(name),
            Mk::Op(_, aop, bop) => {
                kids.entry(aop).or_default().push(name);
                kids.entry(bop).or_default().push(name);
            }
        }
    }
    while let Some(n) = qrdy.pop() {
        let mk = &mks[n];
        println!("eval {n} {mk:?}");
        let v = mk.eval(&vals);
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

fn solve_b(input: &str) -> isize {
    // let mks = parse(EX);
    let mks = parse(input);

    // eval_maybe("pppw", &mks);
    // eval_maybe("sjmn", &mks);
    from_root(&mks)
}

#[allow(dead_code)]
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
        assert_eq!(solve_a(&input()), 169525884255464);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 3247317268284);
    }
}

/*
humn is used in only one place which makes things easier?

We need to work out what the one side of the root value is and then can perhaps
walk down the other side to find a good humn value?

Maybe: first work out which side of 'root' depends on 'humn' (or maybe it's both.)

Then, evaluate the other one to find a number.

Then, keep pushing values down until we know humn.

Relatively simple problem.

*/
