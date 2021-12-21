// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/21

use std::cmp::max;
use std::collections::BTreeMap;

fn main() {
    let (a, b) = solve();
    println!("{}", a);
    println!("{}", b);
}

fn solve() -> (usize, u64) {
    // Player 1 starting position: 7
    // Player 2 starting position: 4
    let sol_a: usize;

    let mut score = [0, 0];
    let mut pos = [7, 4];
    let mut die = 1;
    let mut turn = 0;
    let mut rolls = 0;
    loop {
        let mut tot = 0;
        for _i in 0..3 {
            tot += die;
            die += 1;
            rolls += 1;
            die = (die - 1) % 100 + 1;
            // assert!(die >= 1 && die <= 100);
        }
        pos[turn] = ((pos[turn] + tot) - 1) % 10 + 1;
        score[turn] += pos[turn];
        if score[turn] > 1000 {
            sol_a = score[1 - turn] * rolls;
            break;
        }
        turn = 1 - turn;
    }

    // The states of the universe after each game is completely described by 2 positions and 2
    // scores. All these universes are for our purposes interchangeable.
    //
    // Index by [pos1][pos2][score1][score2] and the value is the count of universes.
    let mut univ: BTreeMap<[usize; 4], u64> = BTreeMap::new();
    univ.insert([7, 4, 0, 0], 1);
    // univ.insert([4, 8, 0, 0], 1);
    // In how many universes did each player win (and then we stopped rolling)
    let mut won = [0; 2];

    // The number of successor universes in which we get each total from rolling 3d3.
    let mut rolls = [0; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                rolls[a + b + c] += 1;
            }
        }
    }
    let goal = 21;

    loop {
        let mut new: BTreeMap<[usize; 4], u64> = BTreeMap::new();
        for ([pos1, pos2, sc1, sc2], count) in univ {
            for (roll1, mul1) in rolls.iter().enumerate().skip(3) {
                let q1 = (pos1 + roll1 - 1) % 10 + 1;
                // assert!(q1 >= 1 && q1 <= 10);
                let sco1 = sc1 + q1;
                if sco1 >= goal {
                    won[0] += count * mul1;
                } else {
                    for (roll2, mul2) in rolls.iter().enumerate().skip(3) {
                        let q2 = (pos2 + roll2 - 1) % 10 + 1;
                        let sco2 = sc2 + q2;
                        if sco2 >= goal {
                            won[1] += count * mul1 * mul2;
                        } else {
                            *new.entry([q1, q2, sco1, sco2]).or_default() += count * mul1 * mul2;
                        }
                    }
                }
            }
        }
        if new.is_empty() {
            break;
        } else {
            univ = new
        }
    }
    let sol_b = max(won[0], won[1]);

    (sol_a, sol_b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve();
        assert_eq!(a, 675024);
        assert_eq!(b, 570239341223618);
    }
}
