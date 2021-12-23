// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/23

use std::convert::TryInto;

use aoclib::shortest_path;

fn main() {
    let (a, b) = solve(&input());
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/23.txt").unwrap()
}

fn cost(c: char) -> isize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn letteridx(c: char) -> usize {
    let i = (c as u32) - ('A' as u32);
    debug_assert!(i < 4);
    i.try_into().unwrap()
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    // So they can never stop outside the rooms; we don't need to model those spaces(?) although we
    // do need to track the cost of moving through them. The hall is then
    // .._._._._..
    // (7 usable spaces.)
    hall: [char; 7],
    rooms: [char; 8],
}

const HALL: usize = 7;

impl State {
    fn new(rooms: &str) -> State {
        State {
            hall: ['.'; 7],
            rooms: rooms.chars().collect::<Vec<char>>().try_into().unwrap(),
        }
    }

    fn to_str(&self) -> String {
        let mut s = String::new();
        s.push(self.hall[0]);
        s.push(self.hall[1]);
        s.push(' ');
        s.push(self.hall[2]);
        s.push(' ');
        s.push(self.hall[3]);
        s.push(' ');
        s.push(self.hall[4]);
        s.push(' ');
        s.push(self.hall[5]);
        s.push(self.hall[6]);
        s.push('\n');
        s.push_str("  ");
        for i in 0..4 {
            s.push(self.rooms[i * 2]);
            s.push(' ');
        }
        s.push('\n');
        s.push_str("  ");
        for i in 0..4 {
            s.push(self.rooms[i * 2 + 1]);
            s.push(' ');
        }
        s
    }

    fn next(self) -> Vec<(State, isize)> {
        let mut r = Vec::new();
        // First, anything in the top cell of the rooms can move out and to the left or right, if
        // the cells they traverse are empty.
        for i in 0..4 {
            let roomi = i * 2;
            let c = self.rooms[roomi];
            if c == '.' {
                continue;
            }
            for hi in 0..HALL {
                if self.hall_clear(i, hi) {
                    let mut hall = self.hall.clone();
                    let mut rooms = self.rooms.clone();
                    rooms[roomi] = '.';
                    hall[hi] = c;
                    r.push((State { hall, rooms }, cost(c) * hall_room_dist(i, hi)));
                }
            }
        }
        // And anything in the bottom of a room can move to the top, if it's empty, but
        // there's no point moving if it's in the right place.
        for i in 0..4 {
            let bot = i * 2 + 1;
            let top = i * 2;
            let c = self.rooms[bot];
            if self.rooms[top] == '.' && c != '.' && letteridx(c) != i {
                let hall = self.hall.clone();
                let mut rooms = self.rooms.clone();
                rooms[top] = c;
                rooms[bot] = '.';
                r.push((State { hall, rooms }, cost(c)));
            }
        }
        // And anything in the hall can move into its own room if the path is clear. If that
        // room is empty it will move to the bottom. If the bottom cell is full and only
        // with the right char, then it can move to the top.
        for hi in 0..HALL {
            let c = self.hall[hi];
            if c == '.' {
                continue;
            }
            let dest = letteridx(c);
            if !self.hall_clear(dest, hi) {
                continue;
            }
            let boti = dest * 2 + 1;
            let topi = dest * 2;
            let botc = self.rooms[boti];
            let topc = self.rooms[topi];
            let move_to = if botc == '.' && topc == '.' {
                boti
            } else if botc == c && self.rooms[topi] == '.' {
                topi
            } else {
                continue;
            };
            let mut hall = self.hall.clone();
            let mut rooms = self.rooms.clone();
            hall[hi] = '.';
            rooms[move_to] = c;
            r.push((
                State { hall, rooms },
                cost(c) * ((move_to == boti) as isize + hall_room_dist(dest, hi)),
            ));
        }
        // println!("next steps: <<<<<< from ");
        // println!("{}", self);
        // for i in &r {
        //     println!("cost={}\n{}", i.1, i.0);
        // }
        // println!(">>>>>>");
        r
    }

    /// True if you can move from that room to the hall position
    /// (The hall position may be occupied)
    fn hall_clear(&self, room: usize, halli: usize) -> bool {
        // .._._._._..
        if halli > 1 + room {
            for i in (room + 2)..halli {
                if self.hall[i] != '.' {
                    return false;
                }
            }
        } else {
            for i in (halli + 1)..=(room + 1) {
                if self.hall[i] != '.' {
                    return false;
                }
            }
        }
        true
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}

fn hall_room_dist(room: usize, hi: usize) -> isize {
    if hi > 1 + room {
        2 + 2 * (hi - (room + 2)) as isize - (hi == 6) as isize
    } else {
        2 + 2 * ((room + 1) - hi) as isize - (hi == 0) as isize
    }
}

fn solve(input: &str) -> (isize, u64) {
    let sol_b = 0;

    // OK so this is like another shortest-path search, maybe? Where the states are the positions?
    // or maybe, the states are the contents of every cell.
    //
    // There are 11 hallway spaces plus 8 room spaces.
    let origin = State::new("CBDADBAC");

    // 12548 too high

    // let origin = State::new("BACDBCDA");
    println!("{}", origin.to_str());
    let dest = State::new("AABBCCDD");
    println!("{}", dest.to_str());
    let sol_a = shortest_path::shortest_distance(origin, dest, State::next);

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exa() {}

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 648681);
        assert_eq!(b, 1302784472088899);
    }
}
