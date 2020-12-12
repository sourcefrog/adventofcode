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

//! Solve https://adventofcode.com/2020/day/12.

pub fn main() {
    println!("12a: {}", solve_a());
    println!("12b: {}", solve_b());
}

fn solve_a() -> isize {
    let input = load_input();
    let mut x = 0;
    let mut y = 0;
    let mut facing = 0;
    for &(inst, arg) in &input {
        match inst {
            'N' => y += arg,
            'S' => y -= arg,
            'E' => x += arg,
            'W' => x -= arg,
            'L' => facing = (facing + arg).rem_euclid(360),
            'R' => facing = (facing - arg).rem_euclid(360),
            'F' => match facing {
                0 => x += arg,
                180 => x -= arg,
                90 => y += arg,
                270 => y -= arg,
                other => panic!("direction {}", other),
            },
            other => panic!("instruction {}", other),
        }
    }
    x.abs() + y.abs()
}

fn solve_b() -> isize {
    let (mut shipx, mut shipy) = (0, 0);
    let (mut wpx, mut wpy) = (10, 1);
    for &(inst, arg) in &load_input() {
        match inst {
            'N' => wpy += arg,
            'S' => wpy -= arg,
            'E' => wpx += arg,
            'W' => wpx -= arg,
            'F' => {
                shipx += wpx * arg;
                shipy += wpy * arg;
            }
            'L' => rot(&mut wpx, &mut wpy, arg),
            'R' => rot(&mut wpx, &mut wpy, (360 - arg).rem_euclid(360)),
            other => panic!("instruction {}", other),
        }
    }
    shipx.abs() + shipy.abs()
}

fn rot(x: &mut isize, y: &mut isize, angle: isize) {
    let ox = *x;
    let oy = *y;
    let (nx, ny) = match angle {
        0 => (ox, oy),
        90 => (-oy, ox),
        180 => (-ox, -oy),
        270 => (oy, -ox),
        other => panic!("rotation {}", other),
    };
    *x = nx;
    *y = ny;
}

fn load_input() -> Vec<(char, isize)> {
    std::fs::read_to_string("input/dec12.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut ch = l.chars();
            let dir = ch.next().unwrap();
            let dist: isize = ch.as_str().parse().unwrap();
            (dir, dist)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 1601);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 13340);
    }
}
