//! https://adventofcode.com/2022/day/22

use itertools::Itertools;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/22.txt").unwrap()
}

struct Map {
    rows: Vec<Vec<bool>>,
    l: Vec<usize>,
}

impl Map {
    fn assert_on_board(&self, x: usize, y: usize) {
        assert!(y < self.rows.len());
        assert!(x >= self.l[y]);
        assert!(x < self.r(y), "x {x} too high for row {y}");
        assert!(self.in_row(x, y));
    }

    fn r(&self, y: usize) -> usize {
        self.l[y] + self.rows[y].len()
    }

    fn w(&self, y: usize) -> usize {
        self.rows[y].len()
    }

    fn in_row(&self, x: usize, y: usize) -> bool {
        x >= self.l[y] && x < self.r(y)
    }

    fn ymin(&self, x: usize) -> usize {
        (0..self.height())
            .filter(|y| self.in_row(x, *y))
            .min()
            .unwrap()
    }

    /// The highest valid (inclusive) x for a row.
    fn xmax(&self, y: usize) -> usize {
        self.l[y] + self.rows[y].len() - 1
    }

    fn ymax(&self, x: usize) -> usize {
        (0..self.height())
            .filter(|y| self.in_row(x, *y))
            .max()
            .unwrap()
    }

    fn col_height(&self, x: usize) -> usize {
        self.ymax(x) + 1 - self.ymin(x)
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn wall_at(&self, x: usize, y: usize) -> bool {
        self.assert_on_board(x, y);
        self.rows[y][x - self.l[y]]
    }

    fn mv(&self, mut x: usize, mut y: usize, dist: usize, dir: usize) -> (usize, usize) {
        for _istep in 0..dist {
            let mut nx = x;
            let mut ny = y;
            let ch = self.col_height(x);
            let ymin = self.ymin(x);
            let xmin = self.l[y];
            match dir {
                0 => {
                    nx = (x - xmin + 1) % self.w(y) + self.l[y];
                }
                1 => {
                    ny = (y - ymin + 1) % ch + ymin;
                }
                2 => {
                    nx = (x - xmin + self.w(y) - 1) % self.w(y) + self.l[y];
                }
                3 => {
                    ny = (y - ymin + ch - 1) % ch + ymin;
                }
                _ => panic!("{dir:?}"),
            }
            if self.wall_at(nx, ny) {
                println!("blocked by wall at {nx},{ny}, remain at {x},{y}");
                break;
            } else {
                println!("step to {nx},{ny}");
                x = nx;
                y = ny;
            }
        }
        (x, y)
    }

    //
    //       top    right
    //       back
    // left  bottom
    // front

    fn mv2(
        &self,
        mut x: usize,
        mut y: usize,
        dist: usize,
        mut dir: usize,
    ) -> (usize, usize, usize) {
        for _istep in 0..dist {
            let nx;
            let ny;
            let ndir;
            // let ch = self.col_height(x);
            let ymin = self.ymin(x);
            let ymax = self.ymax(x);
            let xmin = self.l[y];
            let xmax = self.xmax(y);
            self.assert_on_board(x, y);
            match dir {
                0 => {
                    // right
                    println!("row {y} xmax {xmax}");
                    if x == xmax {
                        println!("move off right edge");
                        assert_eq!(x % 50, 49);
                        if y < 50 {
                            // move from right face to bottom, entering at the +x side, heading -x, y inverted.
                            assert_eq!(x, 149);
                            ndir = 2;
                            nx = 99;
                            ny = 149 - y;
                        } else if y < 100 {
                            // move from back face to right, travelling -y, y coord becomes x.
                            assert_eq!(x, 99);
                            ndir = 3;
                            nx = 100 + (y - 50);
                            ny = 49;
                        } else if y < 150 {
                            // move from bottom to right face, now travelling -x, y inverted.
                            assert_eq!(x, 99);
                            ndir = 2;
                            nx = 149;
                            ny = 49 - (y - 100);
                        } else {
                            assert!(y < 200);
                            assert_eq!(x, 49);
                            // move from front face to bottom, now travelling -y, y coord becomes +x.
                            ndir = 3;
                            nx = 50 + (y - 150);
                            ny = 149;
                        }
                    } else {
                        nx = x + 1;
                        ny = y;
                        ndir = dir;
                    }
                }
                1 => {
                    // down
                    if y == ymax {
                        if x < 50 {
                            // move down from front face to right face, still travelling +y, x coord shifted
                            assert_eq!(y, 199);
                            nx = x + 100;
                            ny = 0;
                            ndir = 1;
                        } else if x < 100 {
                            // move down from bottom face to front face travelling -x, x becomes +y
                            nx = 49;
                            ny = 150 + (x - 50);
                            ndir = 2;
                        } else {
                            assert!(x < 150);
                            // move down from right face to back, travelling -x, x becomes +y
                            nx = 99;
                            ny = 50 + (x - 100);
                            ndir = 2;
                        }
                    } else {
                        nx = x;
                        ny = y + 1;
                        ndir = dir;
                    }
                }
                2 => {
                    // left
                    if x == xmin {
                        if y < 50 {
                            // move from left of top face to left face, travelling +x, y inverted
                            ndir = 0;
                            nx = 0;
                            ny = 100 + (49 - y);
                        } else if y < 100 {
                            // move from back face to left, y becomes x, travelling +y
                            ndir = 1;
                            nx = y - 50;
                            ny = 100;
                        } else if y < 150 {
                            // move from left face to top, travelling +x, y inverted
                            ndir = 0;
                            nx = 50;
                            ny = 49 - (y - 100);
                        } else {
                            assert!(y < 200);
                            // move from front face to top travelling +y, y becomes +x
                            ndir = 1;
                            ny = 0;
                            nx = 50 + (y - 150);
                        }
                    } else {
                        nx = x - 1;
                        ny = y;
                        ndir = dir;
                    }
                }
                3 => {
                    // up
                    if y == ymin {
                        if x < 50 {
                            // from left face to back travelling +x, x becomes +y
                            ndir = 0;
                            nx = 50;
                            ny = 50 + x;
                        } else if x < 100 {
                            // from top face to front travelling +x, x becomes +y
                            ndir = 0;
                            nx = 0;
                            ny = 150 + (x - 50);
                        } else {
                            // from right face to front travelling -y, x becomes +x
                            assert_eq!(y, 0);
                            assert!(x < 150);
                            ndir = 3;
                            nx = x - 100;
                            ny = 199;
                        }
                    } else {
                        ndir = dir;
                        nx = x;
                        ny = y - 1;
                    }
                }
                _ => panic!("{dir:?}"),
            }
            println!("  probe {nx},{ny}");
            if self.wall_at(nx, ny) {
                println!("blocked by wall at {nx},{ny}, remain at {x},{y}");
                break;
            } else {
                println!("step to {nx},{ny} dir {dir}");
                x = nx;
                y = ny;
                dir = ndir;
            }
        }
        (x, y, dir)
    }
}

type Inst = (usize, char);

fn parse(input: &str) -> (Map, Vec<Inst>) {
    let mut map = Map {
        rows: Vec::new(),
        l: Vec::new(),
    };

    let mut ls = input.lines().peekable();
    loop {
        let line = ls.next().unwrap();
        // println!("{line}");
        if line.is_empty() {
            break;
        }
        let l = line.chars().take_while(|c| *c == ' ').count();
        let row = line
            .trim()
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("unexpected {c:?}"),
            })
            .collect_vec();
        map.rows.push(row);
        map.l.push(l);
    }

    let mut line = ls.next().unwrap().to_owned();
    line.push(' ');

    let insts = line
        .split_inclusive(['L', 'R'])
        .map(|s| {
            let (num, dir) = s.split_at(s.len() - 1);
            let num = num.parse::<usize>().unwrap();
            let dir = dir.chars().next().unwrap();
            assert!(
                dir == 'L' || dir == 'R' || dir == ' ',
                "unexpected {dir:?} in {s:?}"
            );
            (num, dir)
        })
        .collect_vec();

    (map, insts)
}

fn solve_a(input: &str) -> usize {
    let (map, insts) = parse(input);
    // y down from the top, x across from the leftmost column
    let mut x = map.l[0];
    let mut y = 0;
    println!("start at {x},{y}");
    let mut dir = 0;
    for (dist, turn) in &insts {
        println!("execute move {dist}{turn}");
        map.assert_on_board(x, y);
        (x, y) = map.mv(x, y, *dist, dir);
        println!("moved to {x},{y}");
        if *turn == 'R' {
            dir = (dir + 1) % 4;
        } else if *turn == 'L' {
            dir = (dir + 3) % 4;
        }
        println!("turn {turn:?} to {dir}");
    }
    (y + 1) * 1000 + (x + 1) * 4 + dir
}

fn solve_b(input: &str) -> usize {
    let (map, insts) = parse(input);
    assert_eq!(map.height(), 200);
    // y down from the top, x across from the leftmost column
    let mut x = map.l[0];
    let mut y = 0;
    println!("start at {x},{y}");
    let mut dir = 0;
    for (dist, turn) in &insts {
        println!("execute move {dist}{turn}");
        map.assert_on_board(x, y);
        (x, y, dir) = map.mv2(x, y, *dist, dir);
        println!("moved to {x},{y}");
        if *turn == 'R' {
            dir = (dir + 1) % 4;
        } else if *turn == 'L' {
            dir = (dir + 3) % 4;
        }
        println!("turn {turn:?} to {dir}");
    }
    // 147271 too high :(
    // also obviously 154087 is too high
    (y + 1) * 1000 + (x + 1) * 4 + dir
}

#[allow(dead_code)]
static EX: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex() {
        assert_eq!(solve_a(EX), 6032);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 196134);
    }

    #[test]
    fn reciprocal_mv2() {
        let (mut map, _) = parse(&input());
        map.rows.iter_mut().for_each(|row| row.fill(false));
        for vx in [0, 49, 50, 99, 100, 149] {
            for vy in [0, 49, 50, 99, 100, 149, 150, 199] {
                if map.in_row(vx, vy) {
                    for dir in 0..4 {
                        println!("---\ncheck point {vx},{vy} dir {dir}");
                        let (nx, ny, ndir) = map.mv2(vx, vy, 1, dir);
                        println!("test move from {vx},{vy} dir {dir} => {nx},{ny},{ndir}");
                        // Coming back in the opposite direction
                        let rdir = (ndir + 2) % 4;
                        let (wx, wy, wdir) = map.mv2(nx, ny, 1, rdir);
                        println!("test return from {nx},{ny},{rdir} => {wx},{wy},{wdir}");
                        // We should return in the opposite direction to what we left
                        assert_eq!((wx, wy, wdir), (vx, vy, (dir + 2) % 4));
                    }
                }
            }
        }
    }

    #[test]
    fn wrapping() {
        // Going 200 steps in any direction should return to the same point
        let (mut map, _) = parse(&input());
        map.rows.iter_mut().for_each(|row| row.fill(false));
        for vx in [0, 49, 50, 99, 100, 149] {
            for vy in [0, 49, 50, 99, 100, 149, 150, 199] {
                if map.in_row(vx, vy) {
                    for vdir in 0..4 {
                        println!("---\ncheck point {vx},{vy},{vdir}");
                        let (nx, ny, ndir) = map.mv2(vx, vy, 200, vdir);
                        println!("test move from {vx},{vy},{vdir} => {nx},{ny},{ndir}");
                        assert_eq!((nx, ny, ndir), (vx, vy, vdir));
                    }
                }
            }
        }
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 146011);
    }
}
