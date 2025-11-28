//! https://adventofcode.com/2022/day/17

use std::fmt;

use aoclib::Matrix;

const MAP_WIDTH: usize = 7;
const TRILLION: usize = 1000000000000;

#[allow(dead_code)]
static EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

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


1589684812539 is too high for part 2.

[2022/src/bin/2022_17.rs:217] initial_rounds = 1808
[2022/src/bin/2022_17.rs:217] initial_growth = 2862
[2022/src/bin/2022_17.rs:217] n_cycles = 573065901
[2022/src/bin/2022_17.rs:217] cycle_growth = 2774
[2022/src/bin/2022_17.rs:217] tail_rounds = 947
[2022/src/bin/2022_17.rs:217] tail_growth = 303
1589684812539


So I think, in the actual problem, the cycles are harder to identify, and so we probably need to look
at the map state as well. To do that we probably need to trim the map to only the squares that can
potentially be reached.

One way to do that is to look for a solid bar across the map. That's not
absolutely guaranteed to happen, although it apparently _does_ happen
periodically in this input.

I guess a more precise version would be: if we can find a continuous path across the
map then no piece can fall below that.

Only looking for blocks in each column would not be enough because of something like this

####...
#......
#......
#......
#......
#....##

Actually, every block there must in some sense be supported from below to have ended up where
it is. We cannot have this

####...
.......
.......
.......
...####
.......

However we could have this, which although it covers every column will let a 2x2 square fall
indefinitely

####...
#......
#......
#......
#......
#..####
#.....#
#.....#
#.....#
#.....#
####..#
#.....#
#.....#

etc.

So maybe the simplest thing is to cut it off when we find a fully populated row.
This will at least be sufficient to know if the reachable map is in the same state?

It seems like the example data is contrived so that it never generates a fully-filled
row across the map even after many thousands of iterations. Maybe we need a different
approach...

We can see there are points in this grid where there's a solid connection across and
so nothing can fall below there. Maybe we could use this kind of reachability to
truncate it. It seems like this is getting a bit complicated though. Maybe there's a
simpler path?

A simpler path to detect cycles? Or something simpler than detecting cycles?

I guess rather than remembering all of the points on the map we could really just
remember where each rock ended up, relative to where it started from. The concatenation
of those values fully determines the content of the map.

OK, new attempt: a more sophisticated attempt to detect where to truncate the map, based
on cells that are reachable from either above or the sides.

OK using detection of cycles based on the reachable map, and after fixing an
annoying off-by-one in the cycle computation, a new guess: 1584927536247.
It is lower than the previous guess that was too high...
*/

fn main() {
    println!("{}", solve_a(&input(), 2022));
    println!("{}", solve_b(&input(), TRILLION));
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

type Rock = Matrix<bool>;

fn rocks() -> Vec<Rock> {
    ROCKS
        .split("\n\n")
        .map(|g| Matrix::from_string_lines(g).map(|c| *c == '#'))
        .collect()
}

fn input() -> String {
    std::fs::read_to_string("input/17.txt").unwrap()
}

fn solve_a(input: &str, rounds: usize) -> usize {
    let mut game = Game::new(input);
    for _i_round in 1..=rounds {
        game.drop_next();
    }
    game.tower_height
}

fn solve_b(input: &str, rounds: usize) -> usize {
    let mut game = Game::new(input);
    let mut rrs = Vec::new();
    let mut initial_rounds = None; // Number of rounds before the cycle is seen to start.
    let mut cycle_rounds = None; // Number of rounds in each cycle.
    let mut cycle_hits = 0;
    for i_round in 0.. {
        let rr = game.drop_next();
        // println!("{} {}", rr.i_rock, rr.i_move);
        // println!("{}", rr.map.to_string());
        // assert_eq!(rr.i_round, i_round);
        if let Some(x) = rrs.iter().rposition(|x| *x == rr) {
            // If the previous rr matches this, then the length would be 1, not 0.
            // println!(
            //     "repeat? cycle of length {} found at i_round={i_round} returns to round={x} move={} rock={} moves{}",
            //     i_round - x,
            //     rr.i_move,
            //     rr.i_rock,
            //     rr.moves
            // );
            if initial_rounds.is_none() {
                // Cycle starts on the first occurrence of rr, initial rounds is the number of
                // rounds prior to that.
                initial_rounds = Some(x);
            }
            cycle_hits += 1;
            match cycle_rounds {
                None => {
                    cycle_rounds = Some(i_round - x);
                }
                Some(rounds) => {
                    assert_eq!(rounds, i_round - x, "cycle length is not stable");
                }
            }
            // println!("{}", rr.map.to_string());
            // println!("previous moves\n{}", rrs[x].map.to_string());
            if cycle_hits > 20 {
                break; // Confident enough that now it continues; really once would be enough.
            }
        }
        rrs.push(rr);
    }
    // So there are three parts to the overall many rounds: some initial rounds, some repetition of
    // cycles, and then a final incomplete cycle.
    let cycle_rounds = cycle_rounds.unwrap();
    let initial_rounds = initial_rounds.unwrap();
    let initial_growth = rrs
        .iter()
        .take(initial_rounds)
        .map(|rr| rr.growth)
        .sum::<usize>();
    let n_cycles = (rounds - initial_rounds) / cycle_rounds;
    let cycle_growth = rrs
        .iter()
        .skip(initial_rounds)
        .take(cycle_rounds)
        .map(|rr| rr.growth)
        .sum::<usize>();
    let tail_rounds = rounds - initial_rounds - n_cycles * cycle_rounds;
    let tail_growth = rrs
        .iter()
        .skip(initial_rounds)
        .take(tail_rounds)
        .map(|rr| rr.growth)
        .sum::<usize>();
    // dbg!(
    //     initial_rounds,
    //     initial_growth,
    //     cycle_rounds,
    //     n_cycles,
    //     cycle_growth,
    //     tail_rounds,
    //     tail_growth
    // );
    initial_growth + n_cycles * cycle_growth + tail_growth
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    /// The content of each column, where element 0 is at the bottom, and
    /// true values are occupied. All cols must be the same length (i.e. height).
    cols: [Vec<bool>; MAP_WIDTH],
}

impl Map {
    fn new() -> Map {
        const EMPTY: Vec<bool> = Vec::new();
        Map {
            cols: [EMPTY; MAP_WIDTH],
        }
    }

    /// If there is a solid row across the map at any point, remove everything below it.
    /// Returns the number of rows removed, which may be zero.
    fn truncate(&mut self) -> usize {
        // Find the lowest reachable row by blocks falling from above or moving sideways.
        let mut rcells = [true; MAP_WIDTH];
        let mut cy = self.grid_height() - 1;
        while cy > 0 {
            // cells on the next line down are reachable if they're not occupied, and
            // the cell directly above is not occupied, or either of the cells
            // diagonally above are reachable. (It's not quite precise.)
            let mut ncells = [false; MAP_WIDTH];
            for x in 0..MAP_WIDTH {
                ncells[x] = !self.get(x, cy - 1)
                    && (rcells[x]
                        || (x > 0) && rcells[x - 1]
                        || (x + 1 < MAP_WIDTH) && rcells[x + 1])
            }
            cy -= 1;
            if !ncells.iter().any(|x| *x) {
                // no cells here are reachable; we can stop
                break;
            }
            rcells = ncells;
        }
        if cy > 0 {
            for i in 0..MAP_WIDTH {
                self.cols[i] = self.cols[i].split_off(cy)
            }
        }
        cy
    }

    /// Return the maximum height of any set cell, or 0 if none are set.
    fn max_block_height(&self) -> usize {
        self.cols
            .iter()
            .flat_map(|col| col.iter().rposition(|x| *x))
            .max()
            .map_or(0, |x| x + 1)
    }

    /// Return the height of the allocated grid.
    fn grid_height(&self) -> usize {
        self.cols[0].len()
    }

    /// True if the cell at (x,y) is set, where y counts up from the base of the map.
    /// If y is off the top of the map this returns false.
    fn get(&self, x: usize, y: usize) -> bool {
        self.cols[x].get(y).copied().unwrap_or(false)
    }

    /// Paint a rock into this map, with the rock's top left at (x,y) where y counts
    /// up from the bottom of the map.
    fn paint(&mut self, rock: &Rock, x: usize, y: usize) {
        // First expand all columns to be tall enough to include row y:
        // if the grid height is 0 and we want to paint in row 0 then we need
        // 1 cell.
        let new_height = y + 1;
        if new_height > self.grid_height() {
            self.cols
                .iter_mut()
                .for_each(|col| col.resize(new_height, false));
        }
        for p in rock.find_values(&true) {
            let mx = p.x as usize + x;
            let my = y.checked_sub(p.y as usize).expect("y off bottom of map");
            assert!(!self.cols[mx][my]);
            self.cols[mx][my] = true;
        }
    }

    /// Return true if a rock at (x,y) would collide with any existing blocks in
    /// this map.
    fn hit_test(&self, rock: &Rock, x: usize, y: usize) -> bool {
        for p in rock.find_values(&true) {
            let mx = p.x as usize + x;
            let my = y.checked_sub(p.y as usize).expect("y off bottom of map");
            if my < self.grid_height() && self.cols[mx][my] {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in (0..self.grid_height()).rev() {
            for col in &self.cols {
                s.push(if col[row] { '#' } else { '.' })
            }
            s.push('\n')
        }
        f.write_str(&s)
    }
}

struct Game {
    /// Next rock to be played
    i_rock: usize,
    i_move: usize,
    i_round: usize,
    rocks: Vec<Rock>,
    moves: Vec<char>,
    map: Map,
    tower_height: usize,
    /// Height above the ground of the bottom of the map.
    base_height: usize,
}

#[derive(PartialEq, Eq)]
struct RoundResult {
    /// How much did the height of the tower increase?
    growth: usize,

    /// Which rock was played?
    i_rock: usize,
    i_move: usize,

    /// What does the reachable map look like after this round?
    map: Map,
}

impl Game {
    fn new(input: &str) -> Game {
        Game {
            i_rock: 0,
            i_move: 0,
            i_round: 1, // 1-based
            rocks: rocks(),
            moves: input.trim().chars().collect(),
            map: Map::new(),
            tower_height: 0,
            base_height: 0,
        }
    }

    /// Drop one rock; consume however many moves it takes for it to settle.
    ///
    /// Updates the game state and returns info about how the move turned out.
    fn drop_next(&mut self) -> RoundResult {
        // y is the position of the top of the rock, measured down from top of the map
        let rock = &self.rocks[self.i_rock];
        let mut y = self.map.max_block_height() + rock.height() + 2;
        let mut x = 2;
        // println!("drop from {x}, {y}\n{}", rock.to_string_lines());
        let orig_block_height = self.map.max_block_height();
        loop {
            let move_ch = self.moves[self.i_move];
            self.i_move = (self.i_move + 1) % self.moves.len();
            let dx = if move_ch == '<' { -1 } else { 1 };
            if x + dx >= 0
                && ((x + dx + rock.width() as isize) <= MAP_WIDTH as isize)
                && !self.map.hit_test(rock, (x + dx) as usize, y)
            {
                // println!("move {move_ch}");
                x += dx;
            } else {
                // println!("can't move {move_ch}");
            }
            if (self.base_height == 0 && y == 0) || self.map.hit_test(rock, x as usize, y - 1) {
                // println!("stopped at {x}, {y}");
                break;
            }
            y -= 1;
            // println!("fall to {x}, {y}");
        }
        self.map.paint(rock, x as usize, y);
        // println!("{}\n", self.map.to_string());
        let growth = self.map.max_block_height() - orig_block_height;
        self.tower_height += growth;
        let r = RoundResult {
            i_rock: self.i_rock,
            i_move: self.i_move,
            growth,
            map: self.map.clone(),
        };
        self.i_rock = (self.i_rock + 1) % self.rocks.len();
        self.i_round += 1;
        let truncated = self.map.truncate();
        if truncated > 0 {
            // println!(
            //     "truncated {truncated} rows in {}, now\n{}",
            //     self.i_round,
            //     self.map.to_string()
            // );
        }
        self.base_height += truncated;
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_a(EX, 2022), 3068);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_b(EX, TRILLION), 1514285714288);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input(), 2022), 3200);
    }

    #[test]
    fn cross_test() {
        assert_eq!(solve_a(&input(), 2022), solve_b(&input(), 2022));
    }

    #[test]
    fn cross_test_longer() {
        let l = 500000;
        assert_eq!(solve_a(&input(), l), solve_b(&input(), l));
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input(), TRILLION), 1584927536247);
    }
}
