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

use aoclib::*;

pub fn main() {
    println!("11a: {}", solve_a());
    println!("11b: {}", solve_b());
}

fn solve_a() -> usize {
    let mut map = Matrix::from_file("input/dec11.txt");
    loop {
        let mut newmap = map.clone();
        let mut changed = false;
        for p in map.points() {
            if map[p] == 'L' && map.neighbors8(p).all(|(_p, &c)| c != '#') {
                newmap[p] = '#';
                changed = true;
            } else if map[p] == '#'
                && map.neighbors8(p).filter(|(_p, &c)| c == '#').count() >= 4
            {
                newmap[p] = 'L';
                changed = true;
            }
        }
        map = newmap;
        if !changed {
            return map.values().filter(|&&c| c == '#').count();
        }
    }
}

fn solve_b() -> usize {
    // This takes about 31ms on my machine.
    //
    // It could probably be made faster by remembering the locations of the
    // visible neighbors of each seat, rather than walking the whole map every
    // time.
    let mut map = Matrix::from_file("input/dec11.txt");
    loop {
        let mut newmap = map.clone();
        let mut changed = false;
        for p in map.points() {
            if map[p] == 'L' && visible_occupied_seats(&map, p) == 0 {
                newmap[p] = '#';
                changed = true;
            } else if map[p] == '#' && visible_occupied_seats(&map, p) >= 5 {
                newmap[p] = 'L';
                changed = true;
            }
        }
        map = newmap;
        if !changed {
            return map.points().filter(|p| map[*p] == '#').count();
        }
    }
}

fn visible_occupied_seats(map: &Matrix<char>, p: Point) -> usize {
    let mut count = 0;
    for (dirx, diry) in Point::DIRECTIONS_8 {
        for i in 1.. {
            let pp = point(p.x + i * dirx, p.y + i * diry);
            match map.try_get(pp) {
                None => break,         // edge of map
                Some('.') => continue, // not a seat
                Some('#') => {
                    // occupied - stop looking
                    count += 1;
                    break;
                }
                Some('L') => break, // unoccupied - stop
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
