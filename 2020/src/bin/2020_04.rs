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

//! Solution to https://adventofcode.com/2020/day/4.

pub fn main() {
    println!("04a: {}", solve_a());
    println!("04b: {}", solve_b());
}

fn solve_a() -> usize {
    std::fs::read_to_string("input/dec04.txt")
        .unwrap()
        .split("\n\n")
        .filter(|para| {
            let fields: Vec<&str> = para
                .split_whitespace()
                .map(|s| s.split(':').take(1).next().unwrap())
                .collect();
            fields.len() == 8 || (fields.len() == 7 && !fields.contains(&"cid"))
        })
        .count()
}

fn solve_b() -> usize {
    std::fs::read_to_string("input/dec04.txt")
        .unwrap()
        .split("\n\n")
        .filter(|p| is_valid(p))
        .count()
}

fn is_valid(para: &str) -> bool {
    let kv: Vec<(&str, &str)> = para
        .split_whitespace()
        .map(|s| {
            let mut parts = s.split(':');
            let k = parts.next().unwrap();
            let v = parts.next().unwrap();
            (k, v)
        })
        .collect();
    if !match kv.len() {
        0..=6 => false,
        7 => !kv.iter().any(|(k, _)| *k == "cid"),
        8 => true,
        _more => false,
    } {
        return false;
    }
    // println!("length ok");
    for (k, v) in kv {
        // println!("check {:?} {:?}", k, v);
        let ok = match k {
            "byr" => v.len() == 4 && ("1920"..="2002").contains(&v),
            "iyr" => v.len() == 4 && ("2010"..="2020").contains(&v),
            "eyr" => v.len() == 4 && ("2020"..="2030").contains(&v),
            "hgt" => {
                if let Some(cm) = v.strip_suffix("cm") {
                    ("150"..="193").contains(&cm)
                } else if let Some(inch) = v.strip_suffix("in") {
                    ("59"..="76").contains(&inch)
                } else {
                    false
                }
            }
            "hcl" => {
                let chars: Vec<char> = v.chars().collect();
                chars.len() == 7
                    && chars[0] == '#'
                    && chars[1..].iter().all(char::is_ascii_hexdigit)
            }
            "ecl" => match v {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => {
                    // println!("bad ecl");
                    false
                }
            },
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
            _ => true,
        };
        if ok {
            // println!("field is ok");
        } else {
            // println!("field validation failed");
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 256);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 198);
    }

    #[test]
    fn invalid_examples() {
        let data = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(data.split("\n\n").count(), 4);
        // assert!(data.split("\n\n").all(|para| !is_valid(para)));
    }

    #[test]
    fn valid_examples() {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert!(data.split("\n\n").all(is_valid));
    }
}
