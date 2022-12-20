//! https://adventofcode.com/2022/day/20

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
    // println!("{}", solve_a(EX));
    println!("{}", solve_a(&input()));
    // println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/20.txt").unwrap()
}

fn solve_a(input: &str) -> isize {
    let n = input
        .lines()
        .map(|l| l.trim().parse::<isize>().unwrap())
        .collect_vec();
    let mut m = n.clone();
    for i in n {
        let cp = m.iter().position(|x| *x == i).unwrap() as isize;
        let mut np;
        if i == 0 {
            println!("0 doesn't move");
            println!("{m:?}");
            continue;
        } else if i > 0 {
            np = (cp as isize + i);
        } else {
            np = cp + i - 1;
        }
        let mlen = m.len() as isize;
        while np > mlen {
            np -= mlen;
        }
        while np < 0 {
            np += mlen;
        }
        let np = np as usize;

        // find the unique number it should move before.
        let move_before = m[(np + 1) % m.len()];
        println!("{i} moves to before {move_before}");
        if move_before == i {
            println!("doesn't move?");
            continue;
        }
        m.remove(cp as usize);
        m.insert(m.iter().position(|x| *x == move_before).unwrap(), i);
        // everything right of cp has been moved one step left, so if we want to insert
        // right of we also need to go one step left.
        // println!("{m:?}");
    }

    // Not 13634 :(
    grove_coord(&m)
}

fn grove_coord(m: &[isize]) -> isize {
    let opos = m.iter().position(|x| *x == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| m[(opos + i) % m.len()])
        .sum::<isize>()
}

fn solve_b(input: &str) -> usize {
    input.len()
}

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
