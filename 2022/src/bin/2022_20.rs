//! https://adventofcode.com/2022/day/20

use std::{collections::HashMap, fmt, rc::Rc};

use itertools::Itertools;

static EX: &str = "\
1
2
-3
3
-2
0
4
";

fn main() {
    println!("{}", solve_a(EX));
    // println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/20.txt").unwrap()
}

/// There is one El for each element in the input; they remain in a vec
/// corresponding to the order they occur in the input.
#[derive(Eq, PartialEq, Debug)]
struct El {
    /// The integer value contained.
    val: isize,
    /// The current position in the ring.
    pos: usize,
}

struct Ring {
    els: Vec<El>,
}

impl Ring {
    fn new(vals: &[isize]) -> Ring {
        Ring {
            els: vals
                .iter()
                .copied()
                .enumerate()
                .map(|(pos, val)| El { pos, val })
                .collect_vec(),
        }
    }

    fn check(&self) {
        for pos in 0..self.els.len() {
            assert_eq!(self.els.iter().filter(|el| el.pos == pos).count(), 1);
        }
    }

    fn len(&self) -> usize {
        self.els.len()
    }

    /// Move the element with the given identity (*not* position) the number of
    /// values forward or back corresponding to its position.
    fn rotate(&mut self, i: usize) {
        let l = self.len() as isize;
        // How much to move?
        let mut m = self.els[i].val;
        println!("move {:?}", &self.els[i]);
        // First for simplicity convert every rotation to rotation of no less
        // than one full cycle to the right.
        while m < 0 {
            m += l;
        }
        m %= l;
        // m now moves it by no more than one cycle relative to its current
        // position, but now there are two possibilities: it moves it
        // further right in the array without wrapping around, or
        self.check();
        // Now there are two possibilities. We might be moving els[i]
    }
}

impl fmt::Debug for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut l = f.debug_list();
        for pos in 0..self.els.len() {
            l.entry(&self.els.iter().find(|el| el.pos == pos).unwrap().val);
        }
        l.finish()
    }
}

fn solve_a(input: &str) -> isize {
    let n = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec();
    let mut ring = Ring::new(&n);
    for i in 0..ring.len() {
        ring.rotate(i);
        println!("{ring:?}");
    }
    // let els: Vec<El> = n
    //     .iter()
    //     .enumerate()
    //     .map(|(pos, &val)| El { val, pos })
    //     .collect();
    // for i in 0..nn {
    //     // Work on els[i]. Move it the right number of steps left or right depending on its val.
    //     let val = els[i].val;
    //     println!("move {val}");
    //     let mut shft = els[i].val;
    //     while shft < 0 {
    //         shft += nn as isize;
    //     }
    //     shft %= nn as isize;
    // }

    // let mut opos = els.iter().find(|el| el.val == 0).unwrap().pos;
    // let mut prod = 1;
    // for _ in [1, 2, 3] {
    //     opos = (opos + 1000) % n.len();
    //     prod *= els.iter().find(|el| el.pos == opos).unwrap().val;
    // }

    // // Not 13634 :(
    // prod
    0
}

// fn solve_b(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_a(EX), 3);
    }

    // #[test]
    // fn solution_a() {
    //     assert_eq!(solve_a(&input()), 9900);
    // }

    // #[test]
    // fn solution_b() {
    //     assert_eq!(solve_b(&input()), 9900);
    // }
}
