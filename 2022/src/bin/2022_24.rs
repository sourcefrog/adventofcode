//! https://adventofcode.com/2022/day/24

use aoclib::shortest_path::ShortestPath;
use itertools::Itertools;

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

/*
For the map we start in 0,0, and need to move to w,h.

The blizzards can be represented as: for each row and column a vec of bits.
It rotates modulo the width or height and this constrains what positions we can move
to next.
    */

struct Map {
    // Width not including inaccessible walls.
    w: usize,

    h: usize,
    /// Vec per col; element i gives whether at clock 0 there is a north-travelling blizard
    /// in row i.
    bl_n: Vec<Vec<bool>>,
    bl_s: Vec<Vec<bool>>,
    bl_e: Vec<Vec<bool>>,
    bl_w: Vec<Vec<bool>>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let lines = input.trim().lines().collect_vec();
        let l = lines[0];
        assert!(l.starts_with("#.#"));
        let w = l.len() - 2;
        let h = lines.len() - 2;
        let mut bl_n = vec![vec![false; h]; w];
        let mut bl_s = vec![vec![false; h]; w];
        let mut bl_w = vec![vec![false; w]; h];
        let mut bl_e = vec![vec![false; w]; h];
        for (y, l) in lines[1..=h].iter().enumerate() {
            assert!(l.starts_with('#'));
            for (x, c) in l.chars().skip(1).take_while(|c| *c != '#').enumerate() {
                match c {
                    '>' => bl_e[y][x] = true,
                    '<' => bl_w[y][x] = true,
                    '^' => bl_n[x][y] = true,
                    'v' => bl_s[x][y] = true,
                    '.' => (),
                    _ => panic!("{c:?}"),
                }
            }
        }
        assert!(lines[h + 1].ends_with("###.#"), "{:?}", lines[h]);
        Map {
            w,
            h,
            bl_n,
            bl_s,
            bl_e,
            bl_w,
        }
    }

    fn bliz_at(&self, clock: usize, x: usize, y: usize) -> char {
        let h = self.h;
        let w = self.w;
        let yclk = clock % h;
        let xclk = clock % w;
        let mut cs = Vec::new();
        if self.bl_n[x][(h + y + yclk) % h] {
            cs.push('^');
        }
        if self.bl_s[x][(h + y - yclk) % h] {
            cs.push('v');
        }
        if self.bl_e[y][(w + x - xclk) % w] {
            cs.push('>');
        }
        if self.bl_w[y][(w + x + xclk) % w] {
            cs.push('<');
        }
        match cs.len() {
            0 => '.',
            1 => cs[0],
            n => char::from_digit(n as u32, 10).unwrap(),
        }
    }

    #[allow(dead_code)]
    fn draw(&self, clock: usize) -> String {
        let mut s = String::new();
        s.push_str("#.");
        for _ in 0..self.w {
            s.push('#');
        }
        s.push('\n');

        for y in 0..self.h {
            s.push('#');
            for x in 0..self.w {
                s.push(self.bliz_at(clock, x, y));
            }
            s.push('#');
            s.push('\n');
        }

        for _ in 0..self.w {
            s.push('#');
        }
        s.push_str(".#");
        s.push('\n');

        s
    }

    fn draw_state(&self, state: &State) -> String {
        let mut s = String::new();
        s.push('#');
        if state.place == Place::Start {
            s.push('E');
        } else {
            s.push('.');
        }
        for _ in 0..self.w {
            s.push('#');
        }
        s.push('\n');

        for y in 0..self.h {
            s.push('#');
            for x in 0..self.w {
                let blz = self.bliz_at(state.clock, x, y);
                match state.place {
                    Place::Point(px, py) if (px, py) == (x, y) => {
                        assert_eq!(blz, '.');
                        s.push('E');
                    }
                    _ => {
                        s.push(blz);
                    }
                }
            }
            s.push('#');
            s.push('\n');
        }

        for _ in 0..self.w {
            s.push('#');
        }
        if state.place == Place::End {
            s.push('E');
        } else {
            s.push('.');
        }
        s.push('#');
        s.push('\n');

        s
    }

    /// Coords of places neighboring a place, including staying still.
    fn nbrs(&self, place: Place) -> Vec<Place> {
        match place {
            Place::Start => vec![Place::Point(0, 0), Place::Start],
            Place::End => vec![Place::End, Place::Point(self.w - 1, self.h - 1)],
            Place::Point(x, y) => {
                let mut v = Vec::new();
                v.push((x, y));
                if x + 1 < self.w {
                    v.push((x + 1, y))
                }
                if x > 0 {
                    v.push((x - 1, y))
                }
                if y + 1 < self.h {
                    v.push((x, y + 1))
                }
                if y > 0 {
                    v.push((x, y - 1))
                }
                let mut places: Vec<Place> =
                    v.into_iter().map(|(x, y)| Place::Point(x, y)).collect();
                if x + 1 == self.w && y + 1 == self.h {
                    places.push(Place::End);
                } else if (x, y) == (0, 0) {
                    places.push(Place::Start);
                }
                places
            }
        }
    }

    /// Places that are valid moves from current clock; depending what will be
    /// occupied in the next clock.
    fn moves(&self, st: &State) -> Vec<(State, usize)> {
        let mut mvs = Vec::new();
        let clock = st.clock + 1;

        for place in self.nbrs(st.place) {
            if let Place::Point(nx, ny) = place {
                if self.bliz_at(clock, nx, ny) == '.' {
                    mvs.push((State { clock, place }, 1));
                }
            } else {
                mvs.push((State { clock, place }, 1));
            }
        }
        // println!("from {st:?} generate {mvs:#?}");
        mvs
    }
    /// Return (x,y) coords including for start/end.
    fn to_coords(&self, place: Place) -> (isize, isize) {
        match place {
            Place::Point(x, y) => (x as isize, y as isize),
            Place::Start => (0, -1),
            Place::End => (self.w as isize - 1, self.h as isize),
        }
    }

    fn manhattan_distance(&self, p1: Place, p2: Place) -> usize {
        let (x1, y1) = self.to_coords(p1);
        let (x2, y2) = self.to_coords(p2);
        let dist = (x1 - x2).abs() + (y1 - y2).abs();
        assert_eq!(dist == 0, p1 == p2, "invalid 0 distance for {p1:?}, {p2:?}");
        dist as usize
    }
}

#[derive(Eq, Debug, PartialEq, Ord, Hash, PartialOrd, Clone, Copy)]
enum Place {
    Start,
    End,
    Point(usize, usize),
}

#[derive(Eq, Debug, PartialEq, Ord, Hash, PartialOrd, Clone, Copy)]
struct State {
    clock: usize,
    place: Place,
}

fn input() -> String {
    std::fs::read_to_string("input/24.txt").unwrap()
}

fn find_path(map: &Map, clock: usize, start: Place, end: Place) -> ShortestPath<State, usize> {
    ShortestPath::find_astar(
        &State {
            clock,
            place: start,
        },
        |st| map.manhattan_distance(st.place, end),
        |st| map.moves(st),
    )
    .expect("no path from {start:?} to {end:?} found")
}

fn solve_a(input: &str) -> usize {
    let map = Map::parse(input);
    let path = find_path(&map, 0, Place::Start, Place::End);
    println!("{:#?}", path.path().collect_vec());
    for st in path.path() {
        println!("clock {}\n{}\n", st.clock, map.draw_state(st));
    }
    println!("{:#?}", path.stats());
    path.distance()
}

#[allow(dead_code)]
fn demo(input: &str) -> usize {
    let map = Map::parse(input);
    for clk in 0..6 {
        println!("{}", map.draw(clk));
    }
    input.len()
}

fn solve_b(input: &str) -> usize {
    let map = Map::parse(input);
    let path = find_path(&map, 0, Place::Start, Place::End);
    let clock = path.final_point().clock;
    assert_eq!(path.distance(), clock);
    let path = find_path(&map, clock, Place::End, Place::Start);
    let clock = path.final_point().clock;
    let path = find_path(&map, clock, Place::Start, Place::End);
    let clock = path.final_point().clock;
    clock
}

#[allow(dead_code)]
static SMOL: &str = "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#   
";

#[allow(dead_code)]
static EX: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 232);
    }

    #[test]
    fn ex2() {
        assert_eq!(solve_b(EX), 54);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 715);
    }
}
