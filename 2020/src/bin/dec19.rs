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
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::error::*;
use nom::multi::*;
use nom::sequence::*;
use nom::*;

pub fn main() {
    println!("19a: {}", solve_a());
    println!("19b: {}", solve_b());
}

type RuleNum = usize;

#[derive(Debug, Clone)]
enum Rule {
    Undef,
    Literal(char),
    Alt(Vec<Vec<RuleNum>>),
}

fn solve_a() -> usize {
    solve_type_a(&input())
}

fn input() -> String {
    std::fs::read_to_string("input/dec19.txt").unwrap()
}

fn solve_b() -> isize {
    let (rules, vals) = parse_input(&input()).unwrap().1;
    // we have "0: 8 11" so this means x repeats of 42 followed by y repeats of 31, where x > y
    // and both are >1.
    let mut count = 0;
    for v in vals {
        let (n42, rest) = count_matches(42, &rules, &v);
        let (n31, rest31) = count_matches(31, &rules, rest);
        // println!( "for value {} n42={} n31={} rest31={:?}", v.iter().collect::<String>(), n42, n31, rest31);
        if n42 > 0 && n31 > 0 && n42 > n31 && rest31.is_empty() {
            count += 1
        }
    }
    count
}

fn count_matches<'a>(rulenum: RuleNum, rules: &[Rule], input: &'a [char]) -> (usize, &'a [char]) {
    let mut rest = input;
    let mut count = 0;
    while let Some(newrest) = apply(rulenum, rules, rest) {
        count += 1;
        rest = newrest;
    }
    (count, rest)
}

fn solve_type_a(input: &str) -> usize {
    let (ruleset, vals) = parse_input(input).unwrap().1;
    vals.iter()
        .filter(|v| {
            matches!(
                apply(0, &ruleset, v),
                Some(rest) if rest.is_empty()
            )
        })
        .count()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Vec<char>>)> {
    all_consuming(separated_pair(
        parse_rules,
        newline,
        many1(terminated(
            map(alpha1, |s: &str| s.chars().collect::<Vec<char>>()),
            newline,
        )),
    ))(input)
}

/// If rule r matches, return the unmatched portion of the string.
fn apply<'a>(num: RuleNum, ruleset: &[Rule], input: &'a [char]) -> Option<&'a [char]> {
    match &ruleset[num] {
        Rule::Undef => panic!(),
        Rule::Literal(c) => {
            if input.is_empty() {
                None
            } else if input[0] == *c {
                Some(&input[1..])
            } else {
                None
            }
        }
        Rule::Alt(v) => {
            'alts: for alt in v {
                let mut rest = input;
                for subrulenum in alt {
                    if let Some(newrest) = apply(*subrulenum, ruleset, rest) {
                        // println!("subrule {} matches with rest {:?}", subrulenum, rest);
                        rest = newrest;
                    } else {
                        // println!("subrule {} failed; try next alt?", subrulenum);
                        continue 'alts;
                    }
                }
                // println!("rule {} matches overall with rest {:?}", num, rest);
                return Some(rest);
            }
            // println!("rule {} fails overall", num);
            None
        }
    }
}

fn parse_rules(l: &str) -> IResult<&str, Vec<Rule>> {
    map(
        many1(terminated(
            separated_pair(
                map_res(digit1, |s: &str| s.parse()),
                tag(": "),
                alt((
                    map(delimited(char('"'), anychar, char('"')), |c| {
                        Rule::Literal(c)
                    }),
                    map(
                        separated_list1(
                            tuple((space0, char('|'), space0)),
                            separated_list1(space1, map(digit1, |s: &str| s.parse().unwrap())),
                        ),
                        Rule::Alt,
                    ),
                )),
            ),
            newline,
        )),
        |numlist: Vec<(usize, Rule)>| {
            let maxrule = numlist.iter().map(|a| a.0).max().unwrap();
            let mut ruleset = vec![Rule::Undef; maxrule + 1];
            for (i, r) in numlist {
                ruleset[i] = r
            }
            ruleset
        },
    )(l)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
        assert_eq!(solve_type_a(&input), 2);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 120);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 350);
    }
}
