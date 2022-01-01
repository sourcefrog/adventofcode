// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/23

use std::convert::TryInto;

use aoclib::shortest_path;

fn main() {
    let b = solve();
    println!("{}", b);
}

fn cost(c: char) -> usize {
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
    rooms: [char; 16],
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
        for row in 0..4 {
            s.push_str("  ");
            for room in 0..4 {
                s.push(self.room_char(room, row));
                s.push(' ');
            }
            s.push('\n');
        }
        s
    }

    fn room_char(&self, room: usize, row: usize) -> char {
        self.rooms[room_idx(room, row)]
    }

    fn next(&self) -> Vec<(State, usize)> {
        let mut r = Vec::new();
        // OK to make this tractable we need to only generate sensible suggestions:
        //
        // A destination is final if its the correct room and all the pods beneath it
        // are also in the correct room.
        //
        // - If we can move from the hall to the final destination for any pod,
        //   we should just do that: it needs to be done and it will never get
        //   any cheaper. And we only need to explore the first such suggestion,
        //   because the others can be done later.
        //
        // - And, if we can move from the top of any other room to a final destination,
        //   we should just do that immediately for similar reasons. (This is just an
        //   optimization for moving through the hall, but probably a good one. Not implemented
        //   yet.)
        //
        // - Otherwise, we can try moving pods from the top of every room, that are
        //   not final, into various locations in the hall.
        //
        // There's no point moving one step or more up or down within a room: this can
        // only usefully be a prelude to moving out of the room.
        let mut room_is_clean = [true; 4];
        let mut bottom_space_in_room = [None; 4];
        let mut top_pod_in_room = [None; 4];
        for room in 0..4 {
            for row in 0..4 {
                let c = self.room_char(room, row);
                if c != '.' && letteridx(c) != room {
                    room_is_clean[room] = false;
                }
                if c == '.' {
                    bottom_space_in_room[room] = Some(row);
                } else if top_pod_in_room[room].is_none() {
                    top_pod_in_room[room] = Some(row);
                }
            }
        }

        // Anything in the hall can move into its own room if the path is clear
        // and the room is clean. If we see any moves like this, we should just
        // greedily take them: they're right, there's no cheaper way to do it,
        // and they don't constrain later actions.
        for hi in 0..HALL {
            let c = self.hall[hi];
            if c == '.' {
                continue;
            }
            let room = letteridx(c);
            if !self.hall_clear(room, hi) || !room_is_clean[room] {
                continue;
            }
            if let Some(row) = bottom_space_in_room[room] {
                return vec![self.move_hall_to_room(hi, room, row)];
            }
        }
        // The top pod in any room, if it's not in a clean room, can move into anywhere in the
        // hall.
        for room in 0..4 {
            if room_is_clean[room] {
                continue;
            }
            if let Some(row) = top_pod_in_room[room] {
                let c = self.room_char(room, row);
                assert_ne!(c, '.');
                for hi in 0..HALL {
                    if self.hall[hi] == '.' && self.hall_clear(room, hi) {
                        r.push(self.move_room_to_hall(room, row, hi));
                    }
                }
            }
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

    #[must_use]
    fn move_hall_to_room(&self, hi: usize, room: usize, row: usize) -> (State, usize) {
        let c = self.hall[hi];
        assert!(('A'..='D').contains(&c));
        assert!(self.room_char(room, row) == '.');
        for lower in (row + 1)..4 {
            assert_eq!(self.room_char(room, lower), c);
        }
        let mut hall = self.hall.clone();
        let mut rooms = self.rooms.clone();
        hall[hi] = '.';
        rooms[room_idx(room, row)] = c;
        (
            State { hall, rooms },
            cost(c) * (hall_room_dist(room, hi) + row),
        )
    }

    #[must_use]
    fn move_room_to_hall(&self, room: usize, row: usize, hi: usize) -> (State, usize) {
        let c = self.room_char(room, row);
        assert_ne!(c, '.');
        assert_eq!(self.hall[hi], '.');
        let mut hall = self.hall.clone();
        let mut rooms = self.rooms.clone();
        rooms[room_idx(room, row)] = '.';
        hall[hi] = c;
        (
            State { hall, rooms },
            cost(c) * (hall_room_dist(room, hi) + row),
        )
    }
}

fn room_idx(room: usize, row: usize) -> usize {
    assert!(room < 4, "{}", room);
    assert!(row < 4, "{}", row);
    row * 4 + room
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}

fn hall_room_dist(room: usize, hi: usize) -> usize {
    if hi > 1 + room {
        2 + 2 * (hi - (room + 2)) - (hi == 6) as usize
    } else {
        2 + 2 * ((room + 1) - hi) - (hi == 0) as usize
    }
}

fn solve() -> usize {
    // OK so this is like another shortest-path search, maybe? Where the states are the positions?
    // or maybe, the states are the contents of every cell.
    //
    // There are 11 hallway spaces plus 8 room spaces.
    let origin = State::new("CDDADCBADBACBABC");

    // 12548 too high
    // B: 52428 is too high

    // let origin = State::new("BCBDDCBADBACADCA");
    println!("{}", origin.to_str());
    let dest = State::new("ABCDABCDABCDABCD");
    println!("{}", dest.to_str());
    shortest_path::shortest_distance(&origin, |p| *p == dest, State::next)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exb() {
        let origin = State::new("BCBDDCBADBACADCA");
        println!("{}", origin.to_str());
        let dest = State::new("ABCDABCDABCDABCD");
        println!("{}", dest.to_str());
        let sol_b = shortest_path::shortest_distance(&origin, |p| *p == dest, State::next);
        assert_eq!(sol_b, 44169);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve(), 50492);
    }
}
