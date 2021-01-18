// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_imports)]

use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::error::*;
use nom::multi::*;
use nom::sequence::*;
use nom::*;

pub fn main() {
    println!("18a: {}", solve_a());
    println!("18b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut tot: usize = 0;
    for l in std::fs::read_to_string("input/dec18.txt").unwrap().lines() {
        let expr = parse(&l);
        println!("{:?}", expr);
        let val = eval(&expr);
        println!("=> {}", val);
        tot += val;
    }
    tot
}

fn solve_b() -> usize {
    let mut tot: usize = 0;
    for l in std::fs::read_to_string("input/dec18.txt").unwrap().lines() {
        let expr = parse(&l);
        println!("{:?}", expr);
        let val = eval_b(&expr);
        println!("=> {}", val);
        tot += val;
    }
    tot
}

#[derive(Debug)]
enum Term {
    Number(usize),
    Parens(Expr),
}
use Term::*;

#[derive(Debug)]
enum TermOrOp {
    Term(Term),
    Op(char),
}

impl TermOrOp {
    fn as_term(&self) -> &Term {
        match self {
            TermOrOp::Term(t) => t,
            _ => panic!(),
        }
    }

    fn as_op(&self) -> char {
        match self {
            TermOrOp::Op(c) => *c,
            _ => panic!(),
        }
    }
}

type Expr = Vec<TermOrOp>;

impl Term {
    fn eval(&self) -> usize {
        match self {
            Number(x) => *x,
            Parens(expr) => eval(expr),
        }
    }
    fn eval_b(&self) -> usize {
        match self {
            Number(x) => *x,
            Parens(expr) => eval_b(expr),
        }
    }
}

fn eval(expr: &Expr) -> usize {
    let mut it = expr.iter();
    assert!(expr.len() >= 3);
    // dbg!(expr.len());
    assert!(expr.len() % 2 == 1);
    let mut accum: usize = it.next().unwrap().as_term().eval();
    while let Some(orop) = it.next() {
        let b = it.next().unwrap();
        let bval = b.as_term().eval();
        match orop.as_op() {
            '*' => accum *= bval,
            '+' => accum += bval,
            _ => panic!(),
        }
    }
    accum
}

fn eval_b(expr: &Expr) -> usize {
    let terms: Vec<&Term> = expr.iter().step_by(2).map(|o| o.as_term()).collect();
    let ops: Vec<char> = expr.iter().skip(1).step_by(2).map(|o| o.as_op()).collect();
    let term_vals: Vec<usize> = terms.iter().map(|t| t.eval_b()).collect();
    dbg!(&term_vals);
    let mut tv_iter = term_vals.into_iter();
    dbg!(&ops);

    let mut to_mul: Vec<usize> = vec![tv_iter.next().unwrap()];
    for ch in ops.iter() {
        let next = tv_iter.next().unwrap();
        dbg!(next, &to_mul);
        match *ch {
            '+' => {
                let a = to_mul.pop().unwrap() + next;
                to_mul.push(a);
            }
            '*' => to_mul.push(next),
            _ => panic!(),
        }
    }
    to_mul.iter().product()
}

fn parse_term(s: &str) -> IResult<&str, Term> {
    alt((
        map(digit1, |d: &str| Number(d.parse().unwrap())),
        delimited(char('('), map(parse_expr, |ex| Parens(ex)), char(')')),
    ))(s)
}

fn parse_expr(s: &str) -> IResult<&str, Expr> {
    separated_list1(
        space1,
        alt((
            map(parse_term, |t| TermOrOp::Term(t)),
            map(one_of("+*"), |c| TermOrOp::Op(c)),
        )),
    )(s)
}

fn parse(s: &str) -> Expr {
    parse_expr(s).unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 209335026987)
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 33331817392479)
    }

    #[test]
    fn examples_b() {
        assert_eq!(eval_b(&parse("1 + (2 * 3) + (4 * (5 + 6))")), 51);
        assert_eq!(eval_b(&parse("2 * 3 + (4 * 5)")), 46);

        assert_eq!(eval_b(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 1445);
        assert_eq!(
            eval_b(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669060
        );
        assert_eq!(
            eval_b(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
            23340
        );
    }
}
