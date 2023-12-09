use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;
use num_integer::lcm;

static YEAR: &str = "2023";
static DAY: &str = "08";

/// Map from node name to the left and right neighbors.
type Map<'s> = HashMap<&'s str, [&'s str; 2]>;

fn main() {
    let input = &input();
    println!("{YEAR}_{DAY} a {}", solve_a(input));
    println!("{YEAR}_{DAY} b {}", solve_b(input));
}

fn input() -> String {
    read_to_string(format!("{YEAR}/input/{DAY}.txt"))
        .or_else(|_| read_to_string(format!("input/{DAY}.txt")))
        .unwrap()
}

fn solve_a(input: &str) -> usize {
    let (turns, nodes) = parse(input);
    let mut turns = turns.into_iter().cycle();
    let mut pos = "AAA";
    let mut steps = 0;
    while pos != "ZZZ" {
        pos = nodes[pos][turns.next().unwrap()];
        steps += 1;
    }
    steps
}

fn solve_b(input: &str) -> usize {
    /* It seems that naively walking them all one step at a time takes too
     * long, which is not too surprising for AoC, although maybe a bit
     * surprising this early in the game...
     *
     * It seems like eventually the paths must all cycle? However, cycling
     * means not only being at the same position but also at the same clock
     * tick: unless the clock is repetitive, but at a glance it does not
     * seem to be...
     *
     * Interestingly, almost unbelievably, we're always getting multiple hits
     * at clock=280 within the turn list, i.e. on the last step before it
     * repeats. This must be rigged in the input?
     *
     * So, then, I think we can work in terms of whole cycles through the list, and
     * see how many full passes through the instructions does it take
     * before each node cycles?
     *
     * It turns out, too, that on this input data each path only ends up cycling through
     * a single Z node, even though that doesn't seem to be necessarily true
     * in the general problem, although it is strongly hinted by saying there
     * are as many points ending in Z as in A.
     *
     * Surprisingly, again, they all seem to have zero phase offset, passing through
     * Z nodes in cycles aligned on step 0. So, is this simply the LCM of
     * all these cycles?
     */
    let (turns, nodes) = parse(input);
    // dbg!(turns.len());
    let cursors = nodes
        .keys()
        .cloned()
        .filter(|name| name.ends_with('A'))
        .collect_vec();
    // dbg!(cursors.len());
    let mut cycles: Vec<usize> = Vec::new(); // len of each cursor's cycle
    for (i, start) in cursors.iter().enumerate() {
        let mut clock = 0; // Absolute number of steps
        let mut pos = *start;
        let mut first_z_clock = None; // Clock at which we first saw a z
        let mut first_z_pos = None; // Clock at which we second saw a z
        loop {
            if is_z(pos) {
                if first_z_clock.is_none() {
                    first_z_clock = Some(clock);
                    first_z_pos = Some(pos);
                } else {
                    assert_eq!(first_z_pos, Some(pos));
                    break;
                }
            }
            pos = nodes[pos][turns[clock % turns.len()]];
            clock += 1;
        }
        let phase = first_z_clock.unwrap();
        let cycle = clock - phase;
        println!("cursor {i} phase {phase} cycle {cycle}");
        assert_eq!(phase, cycle); // seems to be true in this input
        cycles.push(cycle);
    }
    /* Just out of curiosity, are they all primes? Not quite: they
     * all have a common factor of 281, the number of turns in the input, but
     * they do have a prime number of cycles.*/
    for (i, cycle) in cycles.iter().enumerate() {
        println!(
            "cursor {i} cycle {cycle} primes are {:?}",
            num_prime::nt_funcs::factorize64(*cycle as u64)
        );
    }
    cycles.iter().fold(1, |a, b| lcm(a, *b))
}

fn is_z(pos: &str) -> bool {
    pos.ends_with('Z')
}

fn check_name(s: &str) {
    debug_assert!(s.len() == 3, "{s:?}");
    debug_assert!(
        s.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()),
        "{s:?}"
    );
}

fn parse(input: &str) -> (Vec<usize>, Map) {
    let mut lines = input.lines();
    let turns = lines
        .next()
        .expect("List of turns")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("{c:?}"),
        })
        .collect_vec();
    lines.next();
    let mut map: Map = HashMap::new();
    for l in lines {
        let here = &l[0..3];
        check_name(here);
        let left = &l[7..10];
        check_name(left);
        let right = &l[12..15];
        check_name(right);
        assert!(
            map.insert(here, [left, right]).is_none(),
            "node {here:?} occurred twice?"
        )
    }
    (turns, map)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_2() {
        let input = indoc! {"\
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(solve_b(input), 6);
    }

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), 14893);
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), 10241191004509);
    }
}
