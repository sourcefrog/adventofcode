//! https://adventofcode.com/2022/day/17

#![allow(dead_code)]

use std::cmp::{max, min};

use aoclib::Matrix;
use itertools::Itertools;

/* Thoughts on part 2:

Premise: there is a solution that takes less than a trillion steps; there always is.

Hypothesis: the simulation eventually enters a cycle; we could determine the amount of growth
added by the cycle and the length of the cycle. Add that to the growth prior
to entering the cycle and the growth from any incomplete final cycle and
that will give the total growth.

Define "move" as one of the < or > characters.

Define "round" as the process of one single rock falling; it consumes 3 or more moves.
At least 3 because there are always 3 empty rows between the new rock and the top
of any existing rocks.

The state of the simulation is determined by:

 * The state of the reachable upper part of the board, which determines where new
   pieces can fit.

 * The next rock to fall, in 0..=4.

 * The position in the repetitive set of moves.

As a result if we ever return to being ready to play the same set of moves with the
same rock next to fall, and the board looks substantially identical, then the
moves will repeat forever.

The difficulty seems to be in describing whether the state of the board is
substantially identical. We can't just look at the top position in each column
because it's possible for pieces to move sideways into overhangs.

We also can't keep the whole history because it would be very long and also
there could be differences far back in history that don't really affect whether
future plays will cycle.

Option: We could arbitrarily choose some height assumed to be sufficiently
large that every new falling piece will fit in it.

Option: We could look, like in Tetris, for rows that are filled across the board.
Nothing below them can possibly matter. The only problem is whether such rows
will come along often enough or even at all.

Caution: It is not necessarily the case that the cycle must pass through rock 0,
move 0? The initial moves might desynchronize the moves so that no round begins
on move 0?

Option: For each Round we could record a round outcome which says which rock fell,
where it ended up (relative to the previous top?), how much the top grew,
and how many moves were made/consumed?

Hypothesis: If this theory of cycles is true, then we will see some (i_rock, i_move)
states repeating. In fact, it seems impossible that they won't repeat eventually:
there are only 5 rocks and a finite number of moves.

So the challenge is to work out how much of the map we have to remember to be sure
that a cycle is occurring. Maybe start by truncating the board when there is a
row with all cells set.

Experimentation shows that on this input at least, it gets to move 0 and rock 3
after 1749 initial rounds and growth of 2773, and then returns to move 0 and rock 3
after each 1725 rounds, each time growing by 2734.

I suppose it's not strictly proved that this is stable rather than metastable but
since this pattern persists for more than 100 repetitions it seems extremely unlikely
that it's not perfectly repeating. So, perhaps we don't really need to compare
on the map state.

Assume: the cycle does repeat in this way.

(1e12 - 1749) / 1725 = 579_710_143 whole cycles, for growth of 1584927530962.

(1e12 - 1749) % 1725 leaves a residue of 1576 moves, so we need to know how much it
grows then. In 1576 moves after starting the first repeating cycle we grow 2510 units.

So, hopefully, the total is 2773 + 1584927530962 + 2510...

So I guessed 1584927536245 but that is too low.

A good way to check this method would be to check the results on a smaller number that still
includes multiple cycles.

Wonder: Is this just a dumb off-by-one? It seems possible...

For example after 20220 rounds, that's 1749 rounds initially, then 10 cycles, then a remainder
of (20220 - 1749) % 1725 == 1221. That remainder

OK the problem is that I'm printing at the start of the moves and we want to know the height
_after_ the moves.

Checking the position _after_ each move, where is still a cycle, and still hitting move 0 rock 3:

Initially 1748 rounds, growing 2770.

Then cycles of 1725 rounds, growing by 2731.

So for 202200 rounds, that's 117 cycles, and a residue of 375 rounds for 596 growth.

So 2770 + 117 * 2731 + 596?

It's not quite right; the brute force result is 320473, off by 2420?

*/

fn solve_b(input: &str, rounds: usize) -> usize {
    let rocks = rocks();
    // for r in &rocks {
    //     println!("{}\n", r.to_string_lines());
    // }
    const MAP_WIDTH: usize = 7;
    let mh = 600000; // 4000;
    let mut map = Matrix::new(MAP_WIDTH, mh, '.');
    let moves = input.trim().chars().collect_vec();
    let mut i_move = 0;
    let mut top = mh as isize; // y of the highest set piece if any, initally the floor
    let mut i_rock = 0;
    let mut last_move_0_cycle = 0;
    let mut cycle_growth = 0;
    for i_round in 1..=rounds {
        let rock = &rocks[i_rock];
        i_rock = (i_rock + 1) % rocks.len();
        // dbg!(irock);
        // y measured down from top of the map
        let mut y = top - 3 - rock.height() as isize;
        let mut x = 2;
        // println!("start:");
        // draw_temp(rock, &map, x, y);
        loop {
            // dbg!(x, y);
            let move_ch = moves[i_move];
            i_move = (i_move + 1) % moves.len();
            let dx = if move_ch == '<' { -1 } else { 1 };
            // dbg!(dx, rock.width());
            if x + dx >= 0
                && ((x + dx + rock.width() as isize) <= MAP_WIDTH as isize)
                && !intersect(rock, &map, x + dx, y)
            {
                // println!("move {move_ch}");
                x += dx;
            } else {
                // println!("can't move {move_ch}");
            }
            if !on_floor(rock, &map, x, y + 1) && !intersect(rock, &map, x, y + 1) {
                y += 1;
                // println!("fall to {x}, {y}");
            } else {
                // println!("stopped at {x}, {y}");
                break;
            }
            // draw_temp(rock, &map, x, y);
        }
        paint(rock, &mut map, x, y, '#');
        // println!("{}\n", map.to_string_lines());
        if i_move == 0 || (i_round == 1748 + 375) {
            println!(
                "move {i_move}, rock {i_rock}, round {i_round}, rounds in cycle {}, \
growth in cycle {}, top {top}",
                i_round - last_move_0_cycle,
                cycle_growth,
            );
            last_move_0_cycle = i_round;
            cycle_growth = 0;
        } else {
            cycle_growth += max(0, top - y);
        }
        top = std::cmp::min(top, y);
    }
    (mh as isize - top) as usize
}

fn main() {
    // println!("{}", solve_a(&input(), 2022));
    println!("{}", solve_b(&input(), 202200));
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

fn solve_a(input: &str, cycles: usize) -> usize {
    let rocks = rocks();
    for r in &rocks {
        println!("{}\n", r.to_string_lines());
    }
    const MAP_WIDTH: usize = 7;
    let mh = 6000; // 4000;
    let mut map = Matrix::new(MAP_WIDTH, mh, '.');
    let mut moves = input.trim().chars().cycle();
    let mut top = mh as isize; // y of the highest set piece if any, initally the floor
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
                && ((x + dx + rock.width() as isize) <= MAP_WIDTH as isize)
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
    (mh as isize - top) as usize
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

fn on_floor(rock: &Matrix<char>, map: &Matrix<char>, _x: isize, y: isize) -> bool {
    if rock.height() as isize + y > map.height() as isize {
        // println!("touches floor");
        return true;
    }
    false
}

fn intersect(rock: &Matrix<char>, map: &Matrix<char>, x: isize, y: isize) -> bool {
    for (rp, c) in rock.point_values() {
        if *c == '#' {
            let mp = rp.delta(x, y);
            if map[mp] != '.' {
                // println!("hit");
                return true;
            }
        }
    }
    false
}

#[cfg(any())]
fn solve_b(input: &str, cycles: usize) -> usize {
    let rocks = rocks();
    const MAP_WIDTH: usize = 7;
    let moves = input.trim().chars().collect_vec();
    let mut i_rock = 0;
    let mut i_move = 0;
    let mut cols = [0; 7];
    for icycle in 1..=cycles {
        let rock = &rocks[i_rock];
        // y measured above the floo
        let mut y = 2 + cols.iter().copied().max().unwrap() as isize + rock.height() as isize;
        let mut x = 2;
        println!("start at {x},{y}: {cols:?}");
        // draw_temp(rock, &map, x, y);
        loop {
            // dbg!(x, y);
            let move_ch = moves[i_move];
            i_move = (i_move + 1) % moves.len();
            let dx = if move_ch == '<' { -1 } else { 1 };
            // dbg!(dx, rock.width());
            if x + dx >= 0
                && ((x + dx + rock.width() as isize) <= MAP_WIDTH as isize)
                && !intersect_col(rock, &cols, x + dx, y)
            {
                println!("move {move_ch}");
                x += dx;
            } else {
                println!("can't move {move_ch}");
            }

            if rock.height() as isize > y {
                println!("hit the floor at {x}, {y}");
                break;
            } else if intersect_col(rock, &cols, x, y - 1) {
                println!("hit rock at {x}, {y}");
                break;
            } else {
                y -= 1;
                println!("fall to {x}, {y}");
            }
            // draw_temp(rock, &map, x, y);
            // println!("cols {cols:?}");
        }
        paint_cols(rock, &mut cols, x, y);
        // println!("{}\n", map.to_string_lines());
        i_rock = (i_rock + 1) % rocks.len();
        // dbg!(icycle, i_rock, i_move);
        // if i_rock == 0 && i_move == 0 {
        //     panic!("found a repeat after {icycle} moves?");
        // }
    }
    cols.iter().copied().max().unwrap()
}

fn paint_cols(rock: &Matrix<char>, cols: &mut [usize], x: isize, y: isize) {
    for p in rock.find_values(&'#') {
        let cx = (p.x + x) as usize;
        let cy = (y + 1 - p.y) as usize;
        cols[cx] = std::cmp::max(cols[cx], cy);
    }
}

fn intersect_col(rock: &Matrix<char>, cols: &[usize], x: isize, y: isize) -> bool {
    for (p, &c) in rock.point_values() {
        if c != '.' {
            let cy = y - p.y; // map y measures up; matrix measures down
            let cx = p.x + x;
            if cols[cx as usize] > cy as usize {
                return true;
            }
        }
    }
    false
}

fn on_floor_cols(rock: &Matrix<char>, y: isize) -> bool {
    rock.height() as isize > y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            solve_a(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022),
            3068
        );
    }

    #[test]
    fn example_1b() {
        assert_eq!(
            solve_b(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022),
            3068
        );
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input(), 2022), 3200);
    }

    #[test]
    fn solution_a_b() {
        assert_eq!(solve_b(&input(), 2022), 3200);
    }

    #[test]
    fn crosstest() {
        let input = input();
        for cycles in [400] {
            assert_eq!(solve_a(&input, cycles), solve_b(&input, cycles));
        }
    }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 3200);
    // }
}
