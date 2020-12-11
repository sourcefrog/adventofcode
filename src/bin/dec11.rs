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

use adventofcode2020::*;

pub fn main() {
    println!("11a: {}", solve_a());
    println!("11b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut map = Matrix::from_string_lines(&std::fs::read_to_string("input/dec11.txt").unwrap());
    let mut newmap = map.clone();
    let pts: Vec<Point> = map.iter_points().collect();
    loop {
        let mut changed = false;
        for p in pts.clone() {
            if map[p] == 'L' {
                if map.neighbor8_values(p).iter().all(|c| *c != '#') {
                    newmap[p] = '#';
                    changed = true;
                }
            } else if map[p] == '#' {
                if map
                    .neighbor8_values(p)
                    .iter()
                    .filter(|c| **c == '#')
                    .count()
                    >= 4
                {
                    newmap[p] = 'L';
                    changed = true;
                }
            }
        }
        map = newmap.clone();
        if !changed {
            return map.iter_points().filter(|p| map[*p] == '#').count();
        }
    }
}

fn solve_b() -> usize {
    let mut map = Matrix::from_string_lines(&std::fs::read_to_string("input/dec11.txt").unwrap());
    let mut newmap = map.clone();
    let pts: Vec<Point> = map.iter_points().collect();
    loop {
        let mut changed = false;
        for p in pts.clone() {
            if map[p] == 'L' && visible_occupied_seats(&map, p) == 0 {
                newmap[p] = '#';
                changed = true;
            } else if map[p] == '#' && visible_occupied_seats(&map, p) >= 5 {
                newmap[p] = 'L';
                changed = true;
            }
        }
        map = newmap.clone();
        if !changed {
            return map.iter_points().filter(|p| map[*p] == '#').count();
        }
    }
    // 2194 is right
}

fn visible_occupied_seats(map: &Matrix<char>, p: Point) -> usize {
    let mut count = 0;
    for (dirx, diry) in &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ] {
        for i in 1.. {
            let pp = point(p.x + i * dirx, p.y + i * diry);
            match map.try_get(pp) {
                None => break,
                Some('.') => continue,
                Some('#') => {
                    count += 1;
                    break;
                }
                Some('L') => break,
                _other => panic!(),
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(), 2194);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), 1944);
    }
}
