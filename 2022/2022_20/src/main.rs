//! https://adventofcode.com/2022/day/20

use itertools::Itertools;

mod modint;

use crate::modint::{add_isize_mod, add_usize_mod, sub_usize_mod};

#[allow(dead_code)]
static EX: &str = "\
1
2
-3
3
-2
0
4
";

static INPUT: &str = include_str!("../input.txt");

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec()
}

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(INPUT));
    // println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
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
    let l = v.len();
    debug_assert!((0..l).all(|i| v.contains(&i)), "{v:?} is not a permutation");
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

    /// Map the input `x` by `s` elements to the right from its current position,
    /// or to the left if negative.
    #[must_use]
    fn move_element(&self, x: usize, s: isize) -> Perm {
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
        assert!(x < l);
        let mut v = self.0.clone();
        let y = self.0[x];
        let s = s % ll;
        let z = add_isize_mod(y, s, l);
        if s > 0 {
            // Move the next s elements down by one; then input x maps to
            // z.
            let s = s as usize;
            debug_assert_eq!(sub_usize_mod(z, y, l), s);
            for i in v.iter_mut() {
                if sub_usize_mod(*i, y, l) <= s {
                    *i = sub_usize_mod(*i, 1, l);
                }
            }
            v[x] = z;
        } else if s < 0 {
            // Move the prior s elements (with wrapping) right by one, then
            // move to z.
            let s = -s as usize;
            debug_assert_eq!(sub_usize_mod(y, z, l), s);
            for i in v.iter_mut() {
                if sub_usize_mod(y, *i, l) <= s {
                    *i = add_usize_mod(*i, 1, l);
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
    fn apply<T: Copy>(&self, s: &[T]) -> Vec<T> {
        assert_eq!(s.len(), self.0.len());
        let mut v = Vec::with_capacity(self.len());
        v.resize(s.len(), s[0]);
        for (i, x) in self.0.iter().enumerate() {
            v[*x] = s[i];
        }
        v
    }

    // /// The result of another permutation applied to the output of this one.
    // #[must_use]
    // fn combine(&self, other: &Perm) -> Perm {
    //     // For each element in the input of a, it first moves to the output of a,
    //     // then again to where ever other maps that to.
    //     let v = self.0.iter().map(|a| other.0[*a]).collect_vec();
    //     Perm::from_index_vec(v)
    // }

    #[allow(dead_code)]
    fn as_slice(&self) -> &[usize] {
        &self.0
    }
}

fn solve_a(input: &str) -> isize {
    let input = parse(input);
    let l = input.len();
    let mut perm = Perm::new(input.len());
    for (init_pos, value) in input.iter().copied().enumerate() {
        perm = perm.move_element(init_pos, value);
        // println!("{:?}", perm.apply(&input));
    }
    let applied = perm.apply(&input);
    assert_eq!(applied.iter().filter(|i| **i == 0).count(), 1);
    let zero_pos = applied.iter().position(|i| *i == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| applied[add_usize_mod(zero_pos, *i, l)])
        .sum()

    // Not 13634 :(
    // Not -9516 either :(
    // prod
}

// fn solve_b(input: &str) -> usize {
//     input.len()
// }

#[cfg(test)]
mod test {
    use itertools::Itertools;

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
    fn move_left_rotate_whole_cycle() {
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
        let p = Perm::new(7);
        let l = parse(EX);
        assert_eq!(p.apply(&l), [1, 2, -3, 3, -2, 0, 4]);
        let p = p.move_element(0, 1);
        dbg!(&p);
        assert_eq!(p.apply(&l), [2, 1, -3, 3, -2, 0, 4]);
        // Move element with value 2 initially at position 1 by 2
        let p = p.move_element(1, 2);
        dbg!(&p);
        assert_eq!(p.apply(&l), [1, -3, 2, 3, -2, 0, 4]);
        // Move value -3 initially in position 2 by -3.
        let p = p.move_element(2, -3);
        dbg!(&p);
        assert_is_rotation(p.apply(&l), [1, 2, 3, -2, -3, 0, 4]);
        // Move value 3 initially in position 3 by 3.
        let p = p.move_element(3, 3);
        assert_is_rotation(p.apply(&l), [1, 2, -2, -3, 0, 3, 4]);
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

    /// Check one slice is a rotation of the other.
    fn assert_is_rotation<A, B>(a: A, b: B)
    where
        A: AsRef<[isize]>,
        B: AsRef<[isize]>,
    {
        let a = a.as_ref();
        let b = b.as_ref();
        let l = a.len();
        assert_eq!(l, b.len());
        assert!(
            (0..l).any(|r| itertools::equal(a.into_iter().cycle().skip(r).take(l), b)),
            "slices not equal under rotation: {a:?}, {b:?}"
        );
    }
}
