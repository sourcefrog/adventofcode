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

/// Check this is a well-formed permutation: every element up to the length
/// is represented once.
#[allow(dead_code)]
fn check_perm(v: &[usize]) {
    for i in 0..v.len() {
        assert!(v.contains(&i), "element {i} missing from {:?}", v);
    }
}

impl Perm {
    /// Make a new permutation of `len` elements.
    fn new(len: usize) -> Perm {
        Perm((0..len).collect())
    }

    fn from_index_vec(v: Vec<usize>) -> Perm {
        check_perm(&v);
        Perm(v)
    }

    fn as_slice(&self) -> &[usize] {
        &self.0
    }

    /// Map the input `x` by `s` elements to the right from its current position,
    /// or to the left if negative.
    #[must_use]
    fn move_element(&self, x: usize, mut s: isize) -> Perm {
        /*
        If input x is currently routed to output y, then we want to change it to
        output z = (y + s) mod l.

        We can simply change the value for x but we also need to rearrange
        other outputs to make it possible for it to take that position, and to
        fill in y.

        z can end up either greater or less than y depending on both s and
        its value modulo l. If z == y there's nothing to do.

        If z is greater than y then we need to reduce by 1 every mapping
        that currently goes to an output
        in (y+1)..=z. Then, y will have been reassigned, and z will be
        available as an output from x.

        If z is less than y then we need to increase by 1 every mapping
        that currently goes to an output in z..y. That leaves a gap at
        z and we can rewrite x to point there.

        This is not quite right though, because moving to position 0
        put it between the last and first element, and moving to the last
        position would too!

        0 1 2 100 3 4 5

        It is the case that moving l steps returns to the same position.


        */
        let l = self.0.len();
        let ll = l as isize;
        assert!(x < self.0.len());
        let mut v = self.0.clone();
        let y = self.0[x];
        let z = ((y as isize + ll + (s % ll)) % ll) as usize;
        assert!((0..l).contains(&z), "{z}");

        if z > y {
            for i in v.iter_mut() {
                if *i > y && *i <= z {
                    *i -= 1;
                }
            }
            v[x] = z;
        } else if z < y {
            for i in v.iter_mut() {
                if *i >= z && *i < y {
                    *i += 1;
                }
            }
            v[x] = z;
        }
        Perm::from_index_vec(v)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    /// Reorder the elements of a slice according to this permutation.
    #[must_use]
    fn apply<T: Clone + Copy>(&self, s: &[T]) -> Vec<T> {
        assert_eq!(s.len(), self.0.len());
        let mut v = Vec::with_capacity(self.len());
        v.resize(s.len(), s[0]);
        for (i, x) in self.0.iter().enumerate() {
            v[*x] = s[i];
        }
        v
    }

    // The result of another permutation applied to the output of this one.
    #[must_use]
    fn combine(&self, other: &Perm) -> Perm {
        // For each element in the input of a, it first moves to the output of a,
        // then again to where ever other maps that to.
        let v = self.0.iter().map(|a| other.0[*a]).collect_vec();
        Perm::from_index_vec(v)
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
        let l = parse(EX);
        assert_eq!(p1.apply(&l), [1, 2, -3, 3, -2, 0, 4]);
        let p1 = p1.move_element(0, 1);
        dbg!(&p1);
        assert_eq!(p1.apply(&l), [2, 1, -3, 3, -2, 0, 4]);
        // Move element with value 2 initially at position 1 by 2
        let p2 = p1.move_element(1, 2);
        dbg!(&p2);
        assert_eq!(p2.apply(&l), [1, -3, 2, 3, -2, 0, 4]);
        // Move value -3 initially in position 2 by -3.
        let p = p2.move_element(2, -3);
        dbg!(&p);
        assert_eq!(p.apply(&l), [1, 2, 3, -2, -3, 0, 4]);
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
