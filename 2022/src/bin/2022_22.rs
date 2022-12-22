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

// dx, dy; increasing values turn right.
static DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Map {
    fn assert_on_board(&self, x: usize, y: usize) {
        assert!(y < self.rows.len());
        assert!(x >= self.l[y]);
        assert!(x < self.r(y));
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
        println!("{line}");
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
    input.len()
}

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

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);
    // }
}
