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

const CARDPUB: u64 = 15628416;
const DOORPUB: u64 = 11161639;

const MODULUS: u64 = 20201227; // cute

pub fn main() {
    println!("25a: {}", solve_a());
}

fn solve_a() -> u64 {
    let mut val: u64 = 1;
    let mut i = 1;
    let a = loop {
        val = (val * 7) % MODULUS;
        if val == CARDPUB {
            break i;
        }
        i += 1;
    };
    let mut val: u64 = 1;
    for _i in 1..=a {
        val = (val * DOORPUB) % MODULUS;
    }
    val
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 19774660);
    }
}
