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

#![allow(dead_code, unused_imports)]

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{IResult, Parser};

use std::collections::{BTreeMap, BTreeSet};

pub fn main() {
    let (a, b) = solve();
    println!("21a: {}", a);
    println!("21b: {}", b);
}

fn solve() -> (usize, String) {
    solve_on(&load())
}

fn solve_on(s: &str) -> (usize, String) {
    let foods = parse(s);

    // each allergen is in exactly one ingredient.
    // each food that lists an allergen must have it in one of the listed ingredients.
    // each ingredient contains zero ro one allergen
    // when the allergen *is* listed, its ingredient must be somewhere in the list

    let mut alg_to_ingr: BTreeMap<String, String> = BTreeMap::new();
    let mut ingr_to_alg: BTreeMap<String, String> = BTreeMap::new();
    // For each allergen, the possible ingredients.
    let mut poss_ingr: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let all_alg: BTreeSet<String> = foods
        .iter()
        .flat_map(|f| f.allergens.iter())
        .map(ToOwned::to_owned)
        .collect();
    let all_ingr: BTreeSet<String> = foods
        .iter()
        .flat_map(|f| f.ingreds.iter().cloned())
        .collect();

    while alg_to_ingr.len() != all_alg.len() {
        let mut changed = false;
        for food in &foods {
            for alg in &food.allergens {
                if alg_to_ingr.contains_key(alg) {
                    // already solved
                    continue;
                }
                changed = true;
                let mut ings = food.ingreds.clone();
                // remove ingredients with known mappings
                ings.retain(|i| !ingr_to_alg.contains_key(i));
                let en = poss_ingr
                    .entry(alg.clone())
                    .and_modify(|ei|
                        // Only ingredients common to the two foods remain possibilities.
                        ei.retain(|i| ings.iter().find(|j| **j == *i).is_some()))
                    .or_insert_with(|| ings);
                if en.len() == 1 {
                    let final_ing = &en[0];
                    // println!("only one possibility for {}: {}", alg, final_ing);
                    alg_to_ingr.insert(alg.clone(), final_ing.clone());
                    ingr_to_alg.insert(final_ing.clone(), alg.clone());
                } else if en.is_empty() {
                    panic!("eliminated every possible ingredient for {}?", alg);
                }
            }
        }
        if !changed {
            panic!("made no progress?");
        }
    }

    let answer_a = all_ingr
        .iter()
        .filter(|i| !ingr_to_alg.contains_key(*i))
        .map(|i| {
            foods
                .iter()
                .filter(|f| f.ingreds.iter().find(|fi| *i == **fi).is_some())
                .count()
        })
        .sum();
    let answer_b = alg_to_ingr
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join(",");
    (answer_a, answer_b)
}

#[derive(Debug)]
struct Food {
    ingreds: Vec<String>,
    allergens: Vec<String>,
}

fn parse(s: &str) -> Vec<Food> {
    try_parse(s).unwrap().1
}

fn try_parse(s: &str) -> IResult<&str, Vec<Food>> {
    many1(terminated(
        map(
            tuple((
                separated_list1(space1, map(alpha1, ToOwned::to_owned)),
                tag(" (contains "),
                terminated(
                    separated_list1(tag(", "), map(alpha1, ToOwned::to_owned)),
                    tag(")"),
                ),
            )),
            |t| Food {
                ingreds: t.0,
                allergens: t.2,
            },
        ),
        newline,
    ))(s)
}

fn load() -> String {
    std::fs::read_to_string("input/dec21.txt").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &'static str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";

    #[test]
    fn example_a() {
        assert_eq!(solve_on(EXAMPLE), (5, "mxmxvkd,sqjhc,fvjkl".to_owned()));
    }

    #[test]
    fn solution() {
        assert_eq!(
            solve(),
            (
                1685,
                "ntft,nhx,kfxr,xmhsbd,rrjb,xzhxj,chbtp,cqvc".to_owned()
            )
        )
    }
}
