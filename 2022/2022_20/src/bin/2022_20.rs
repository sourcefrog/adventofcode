//! https://adventfield1fcode.field1om/2022/day/20

use itertools::Itertools;

use mbpaoc2022_20::modint::{add_isize_mod, add_usize_mod, sub_usize_mod};

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

static INPUT: &str = include_str!("../../input.txt");

static KEY: isize = 811589153;

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec()
}

fn main() {
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(INPUT));
    println!("{}", solve_b(INPUT));
}

/// A permutation of the elements of an input of given size.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Perm {
    /// The final position of input i.
    ///
    /// The array must always contain all the numbers 0..n: elements are never
    /// lost or duplicated.
    input_pos: Vec<usize>,
}

/// Check this is a well-formed permutation: every element up to the length
/// is represented once.
#[allow(dead_code)]
#[cfg(debug)]
fn check_perm(v: &[usize]) {
    let l = v.len();
    let mut seen = vec![false; l];
    v.iter().for_each(|x| seen[*x] = true);
    debug_assert!(seen.iter().all(|x| *x), "{v:?} is not a permutation");
}

#[allow(dead_code)]
#[cfg(not(debug))]
fn check_perm(_: &[usize]) {}

impl Perm {
    /// Make a new permutation of `len` elements.
    fn new(len: usize) -> Perm {
        Perm {
            input_pos: (0..len).collect(),
        }
    }

    /// Map the input `x` by `s` elements to the right from its current position,
    /// or to the left if negative.
    fn move_element(&mut self, x: usize, s: isize) -> &mut Self {
        /*
        Skipping (l-1) elements returns you to the same position.

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
        let l = self.input_pos.len();
        if l <= 1 {
            return self;
        }
        let ll = l as isize;
        assert!(x < l);
        let y = self.input_pos[x];
        // Skipping just (ll-1) elements would result in no change.
        let s = s % (ll - 1);
        let z = add_isize_mod(y, s, l);
        if s > 0 {
            // Move the next s elements down by one; then input x maps to
            // z.
            let s = s as usize;
            debug_assert_eq!(sub_usize_mod(z, y, l), s);
            self.input_pos.iter_mut().for_each(|i| {
                if *i == y {
                    *i = z
                } else if sub_usize_mod(*i, y, l) <= s {
                    *i = sub_usize_mod(*i, 1, l)
                }
            });
        } else if s < 0 {
            // Move the prior s elements (with wrapping) right by one, then
            // move to z.
            let s = -s as usize;
            debug_assert_eq!(sub_usize_mod(y, z, l), s);
            self.input_pos.iter_mut().for_each(|i| {
                if *i == y {
                    *i = z
                } else if sub_usize_mod(y, *i, l) <= s {
                    *i = add_usize_mod(*i, 1, l)
                }
            });
        }
        self
    }

    fn len(&self) -> usize {
        self.input_pos.len()
    }

    /// Reorder the elements of a slice according to this permutation.
    #[must_use]
    fn apply<T: Copy>(&self, s: &[T]) -> Vec<T> {
        assert_eq!(s.len(), self.input_pos.len());
        let mut v = Vec::with_capacity(self.len());
        v.resize(s.len(), s[0]);
        for (i, x) in self.input_pos.iter().enumerate() {
            v[*x] = s[i];
        }
        v
    }

    // Two permutations are equivalent if the ordering of results is the same modulo
    // some rotation.
    #[cfg(test)]
    fn equivalent(&self, other: &Perm) -> bool {
        let l = self.input_pos.len();
        assert_eq!(l, other.len());
        if l <= 1 {
            return true;
        }
        let d = sub_usize_mod(other.input_pos[0], self.input_pos[0], l);
        self.input_pos
            .iter()
            .zip(&other.input_pos)
            .all(|(a, b)| sub_usize_mod(*b, *a, l) == d)
    }
}

fn solve_a(input: &str) -> isize {
    let input = parse(input);
    let mut perm = Perm::new(input.len());
    for (init_pos, value) in input.iter().copied().enumerate() {
        perm.move_element(init_pos, value);
    }
    grove_coord(&perm, &input)
}

fn solve_b(input: &str) -> isize {
    let input = parse(input).into_iter().map(|i| i * KEY).collect_vec();
    let mut perm = Perm::new(input.len());
    for _round in 0..10 {
        for (init_pos, &value) in input.iter().enumerate() {
            perm.move_element(init_pos, value);
        }
    }
    grove_coord(&perm, &input)
}

fn grove_coord(perm: &Perm, input: &[isize]) -> isize {
    let l = perm.len();
    let applied = perm.apply(&input);
    let zero_pos = applied.iter().position(|i| *i == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| applied[add_usize_mod(zero_pos, *i, l)])
        .sum()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn move_zero() {
        let mut p = Perm::new(5);
        assert_eq!(p.input_pos, [0, 1, 2, 3, 4]);
        p.move_element(2, 0);
        assert_eq!(p.input_pos, [0, 1, 2, 3, 4]);
        assert_eq!(
            p.apply(&[100, 101, 102, 103, 104]).as_slice(),
            [100, 101, 102, 103, 104]
        );
    }

    #[test]
    fn move_right() {
        let mut p = Perm::new(5);
        assert_eq!(p.input_pos, [0, 1, 2, 3, 4]);
        p.move_element(2, 2);
        assert_eq!(p.input_pos, [0, 1, 4, 2, 3]);
        assert_eq!(
            p.apply(&"abcde".chars().collect_vec()),
            "abdec".chars().collect_vec().as_slice(),
        );
    }

    #[test]
    fn move_right_rotate_whole_cycle() {
        let l = 5;
        let mut p = Perm::new(l);
        let p_orig = p.clone();
        for i in 0..5 {
            for m in 0..4 {
                p.move_element(i, m * (l as isize - 1));
                assert!(p.equivalent(&p_orig));
            }
        }
    }

    #[test]
    fn move_left_rotate_whole_cycle() {
        let l = 5;
        let p_orig = Perm::new(l);
        let mut p = p_orig.clone();
        for i in 0..5 {
            for m in 0..4 {
                p.move_element(i, -m * (l as isize - 1));
                assert!(p.equivalent(&p_orig));
            }
        }
    }

    #[test]
    fn move_left() {
        let mut p = Perm::new(5);
        assert_eq!(p.input_pos, [0, 1, 2, 3, 4]);
        p.move_element(2, -1);
        assert_eq!(p.input_pos, [0, 2, 1, 3, 4]);
        assert_eq!(
            p.apply(&"abcde".chars().collect_vec()),
            "acbde".chars().collect_vec(),
        );
    }

    #[test]
    fn ex_1_parts() {
        let mut p = Perm::new(7);
        let l = parse(EX);
        assert_eq!(p.apply(&l), [1, 2, -3, 3, -2, 0, 4]);
        p.move_element(0, 1);
        dbg!(&p);
        assert_eq!(p.apply(&l), [2, 1, -3, 3, -2, 0, 4]);
        // Move element with value 2 initially at position 1 by 2
        p.move_element(1, 2);
        dbg!(&p);
        assert_eq!(p.apply(&l), [1, -3, 2, 3, -2, 0, 4]);
        // Move value -3 initially in position 2 by -3.
        p.move_element(2, -3);
        dbg!(&p);
        assert_is_rotation(p.apply(&l), [1, 2, 3, -2, -3, 0, 4]);
        // Move value 3 initially in position 3 by 3.
        p.move_element(3, 3);
        assert_is_rotation(p.apply(&l), [1, 2, -2, -3, 0, 3, 4]);
    }

    #[test]
    fn ex1() {
        assert_eq!(solve_a(EX), 3);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(INPUT), 10831);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(INPUT), 6420481789383);
    }

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

#[cfg(test)]
mod proptest {
    use ::proptest::prelude::*;

    use super::*;

    proptest! {
         #[test]
         fn reversible(l in 1..10usize, a in 0..10usize, s in -22..22isize) {
             let p_orig = Perm::new(l);
             let mut p=p_orig.clone();
             let a = a % l;
             p.move_element(a, s).move_element(a, -s);
             assert_eq!(p, p_orig);
         }

         #[test]
         fn additive(l in 1..10usize, a in 0..10usize, s in -10..10isize, t in -10..10isize) {
             let mut p = Perm::new(l);
             let mut p3 = p.clone();
             let a = a % l;
             p.move_element(a, s).move_element(a, t);
             p3.move_element(a, s + t);
             assert!(p.equivalent(&p3));
         }
    }
}
