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

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec()
}

fn main() {
    println!("{}", solve_a(EX));
    // println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/20.txt").unwrap()
}

/// A permutation of the elements of an input of given size.
///
/// p.0[i] is the position where the element initally at position i ends up.
/// p.0 must always contain all the successive whole numbers: elements are
/// never lost or duplicated.
#[derive(PartialEq, Eq, Debug)]
struct Perm(Vec<usize>);

impl Perm {
    /// Make a new permutation of `len` elements.
    fn new(len: usize) -> Perm {
        Perm((0..len).collect())
    }

    /// Check this is a well-formed permutation: every element up to the length
    /// is represented once.
    fn check(&self) {
        for i in 0..self.len() {
            assert!(self.0.contains(&i), "element {i} missing from {:?}", self.0);
        }
    }

    fn as_slice(&self) -> &[usize] {
        &self.0
    }

    /// Map the input `x` by `s` elements to the right from its current position,
    /// or to the left if negative.
    #[must_use]
    fn move_element(&self, x: usize, mut s: isize) -> Perm {
        let l = self.0.len();
        assert!(x < self.0.len());
        let mut v = self.0.clone();
        let mut y = x as isize + s;
        // ox is where x is currently routed to.
        let ox = self.0[x];
        if y < 0 {
            y = (y % l as isize) + l as isize;
        }
        y %= l as isize;
        assert!(y >= 0 && y < l as isize);
        let y = y as usize;
        // y is where it should be routed to
        dbg!(x, ox, y);
        if y >= x {
            // The input is:         aaaa x bbbb cccc
            // The output should be: aaaa bbbb y cccc
            // This means: the positions
            // Any of a and b might be empty.
            // This means the position of everything in b is reduced by 1.
            // Then x takes position y.
            // The range of y is: (x+1)..=y.
            // Any input currently in that range is moved left by one.
            for i in v.iter_mut() {
                if *i > ox && *i <= y {
                    *i -= 1;
                }
            }
            v[x] = y;
        } else {
            // The input is aaa bbb x ccc
            // We want: aaa y bbb ccc
            // Everything in bbb moves one position right.
            for i in v.iter_mut() {
                if *i >= y && *i < ox {
                    *i += 1;
                }
            }
            v[x] = y;
        }
        let o = Perm(v);
        o.check();
        o
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    /// Reorder the elements of a slice according to this permutation.
    #[must_use]
    fn apply<T: Clone>(&self, s: &[T]) -> Vec<T> {
        assert_eq!(s.len(), self.0.len());
        let mut v = Vec::with_capacity(self.len());
        for i in 0..(self.0.len()) {
            // Which element of the input comes next?
            // TODO: Don't scan it repeatedly!
            let j = self.0.iter().position(|y| *y == i).unwrap();
            v.push(s[j].clone());
        }
        v
    }

    // The result of another permutation applied to the output of this one.
    #[must_use]
    fn combine(&self, other: &Perm) -> Perm {
        // For each element in the input of a, it first moves to the output of a,
        // then again to where ever other maps that to.
        let v = self.0.iter().map(|a| other.0[*a]).collect_vec();
        let o = Perm(v);
        o.check();
        o
    }
}

fn solve_a(input: &str) -> isize {
    let n = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec();
    let mut perm = Perm::new(n.len());
    for i in 0..n.len() {
        perm = perm
        // TODO: Find which current position corresponds to originally i.
        // perm = perm.move_element(i)
        // ring.rotate(i);
        // println!("{ring:?}");
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
    fn move_zero() {
        let p1 = Perm::new(5);
        assert_eq!(p1.as_slice(), [0, 1, 2, 3, 4]);
        let p2 = p1.move_element(2, 0);
        assert_eq!(p2.as_slice(), [0, 1, 2, 3, 4]);
        assert_eq!(
            p2.apply(&[100, 101, 102, 103, 104]).as_slice(),
            [100, 101, 102, 103, 104]
        );
    }

    #[test]
    fn move_right() {
        let p1 = Perm::new(5);
        assert_eq!(p1.as_slice(), [0, 1, 2, 3, 4]);
        let p2 = p1.move_element(2, 2);
        assert_eq!(p2.as_slice(), [0, 1, 4, 2, 3]);
        assert_eq!(
            p2.apply("abcde".chars().collect_vec().as_slice()),
            "abdec".chars().collect_vec().as_slice(),
        );
    }

    #[test]
    fn move_right_rotate() {
        let p1 = Perm::new(5);
        for i in 0..5 {
            for m in 0..4 {
                assert_eq!(p1.move_element(i, m * 5), p1);
            }
        }
    }

    #[test]
    fn move_left_rotate() {
        let p1 = Perm::new(5);
        for i in 0..5 {
            for m in 0..4 {
                assert_eq!(p1.move_element(i, m * -5), p1);
            }
        }
    }

    #[test]
    fn move_left() {
        let p1 = Perm::new(5);
        assert_eq!(p1.as_slice(), [0, 1, 2, 3, 4]);
        let p2 = p1.move_element(2, -1);
        assert_eq!(p2.as_slice(), [0, 2, 1, 3, 4]);
        assert_eq!(
            p2.apply("abcde".chars().collect_vec().as_slice()),
            "acbde".chars().collect_vec().as_slice(),
        );
    }

    #[test]
    fn ex_1_parts() {
        let p1 = Perm::new(7);
        assert_eq!(p1.apply(&parse(EX)), [1, 2, -3, 3, -2, 0, 4]);
        let p1 = p1.move_element(0, 1);
        dbg!(&p1);
        assert_eq!(p1.apply(&parse(EX)), [2, 1, -3, 3, -2, 0, 4]);
        // Where is the 2 that we now want to map? It was originally in 1.
        let x = p1.0[1];
        dbg!(&x);
        let p2 = p1.move_element(x, 2);
        dbg!(&p2);
        assert_eq!(p2.apply(&parse(EX)), [1, -3, 2, 3, -2, 0, 4]);
    }

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
