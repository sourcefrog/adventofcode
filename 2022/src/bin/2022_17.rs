//! https://adventofcode.com/2022/day/17

use aoclib::Matrix;

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

static ROCKS: &str = "\
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

fn rocks() -> Vec<Matrix<char>> {
    ROCKS
        .split("\n\n")
        .map(|g| Matrix::from_string_lines(g))
        .collect()
}

fn input() -> String {
    std::fs::read_to_string("input/17.txt").unwrap()
}

fn solve_a(input: &str) -> isize {
    let rocks = rocks();
    for r in &rocks {
        println!("{}\n", r.to_string_lines());
    }
    let map_width = 7;
    let mh = 5000; // 4000;
    let mut map = Matrix::new(map_width, mh, '.');
    let mut moves = input.trim().chars().cycle();
    let mut top = mh as isize; // y of the highest set piece if any, initally the floor
    let cycles = 2022;
    for (irock, rock) in (1..=cycles).zip(rocks.iter().cycle()) {
        dbg!(irock);
        // y measured down from top of the map
        let mut y = top - 3 - rock.height() as isize;
        let mut x = 2;
        println!("start:");
        // draw_temp(rock, &map, x, y);
        loop {
            dbg!(x, y);
            let move_ch = moves.next().unwrap();
            let dx = if move_ch == '<' { -1 } else { 1 };
            dbg!(dx, rock.width());
            if x + dx >= 0
                && ((x + dx + rock.width() as isize) <= map_width as isize)
                && !intersect(rock, &map, x + dx, y)
            {
                println!("move {move_ch}");
                x += dx;
            } else {
                println!("can't move {move_ch}");
            }
            if !on_floor(rock, &map, x, y + 1) && !intersect(rock, &map, x, y + 1) {
                y += 1;
                println!("fall to {x}, {y}");
            } else {
                println!("stopped at {x}, {y}");
                break;
            }
            // draw_temp(rock, &map, x, y);
        }
        paint(rock, &mut map, x, y, '#');
        // println!("{}\n", map.to_string_lines());
        top = std::cmp::min(top, y);
    }
    mh as isize - top
}

fn draw_temp(rock: &Matrix<char>, map: &Matrix<char>, x: isize, y: isize) {
    let mut temp_map = map.clone();
    paint(rock, &mut temp_map, x, y, '@');
    println!("{}\n", temp_map);
}

fn paint(rock: &Matrix<char>, map: &mut Matrix<char>, x: isize, y: isize, pc: char) {
    for (rp, &c) in rock.point_values() {
        if c != '.' {
            let mp = rp.delta(x, y);
            assert_eq!(map[mp], '.');
            map[mp] = pc;
        }
    }
}

fn on_floor(rock: &Matrix<char>, map: &Matrix<char>, x: isize, y: isize) -> bool {
    if rock.height() as isize + y > map.height() as isize {
        println!("touches floor");
        return true;
    }
    false
}

fn intersect(rock: &Matrix<char>, map: &Matrix<char>, x: isize, y: isize) -> bool {
    for (rp, c) in rock.point_values() {
        if *c == '#' {
            let mp = rp.delta(x, y);
            if map[mp] != '.' {
                println!("hit");
                return true;
            }
        }
    }
    false
}

fn solve_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_a(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"), 3068);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 9900);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 9900);
    }
}
