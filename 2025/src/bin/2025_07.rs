use itertools::Itertools;

fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve1(input: &str) -> usize {
    let mut lines = input.lines();
    let first = lines.next().unwrap().trim();
    let width = first.len();
    assert!(first.chars().all(|c| c == 'S' || c == '.'));
    let mut beams = vec![false; width];
    let start = first.chars().position(|c| c == 'S').unwrap();
    beams[start] = true;
    let mut split_count = 0;
    for (blanks, line) in lines.tuples() {
        assert!(blanks.trim().chars().all(|c| c == '.'));
        let line = line.trim();
        assert!(
            line.chars().all(|c| c == '.' || c == '^'),
            "unexpected char in {line:?}"
        );
        for split_pos in line.chars().positions(|c| c == '^') {
            // if there is a beam entering this position, it moves to the left and right instead
            if beams[split_pos] {
                beams[split_pos.checked_sub(1).unwrap()] = true;
                beams[split_pos] = false;
                beams[split_pos + 1] = true;
                split_count += 1;
            }
        }
    }
    split_count
}

fn solve2(input: &str) -> usize {
    let mut lines = input.lines();
    let first = lines.next().unwrap().trim();
    let width = first.len();
    assert!(first.chars().all(|c| c == 'S' || c == '.'));
    let mut beams = vec![0; width]; // the number of distinct paths leading to this column in the current row
    let start = first.chars().position(|c| c == 'S').unwrap();
    beams[start] = 1;
    for (blanks, line) in lines.tuples() {
        assert!(blanks.trim().chars().all(|c| c == '.'));
        let line = line.trim();
        assert!(
            line.chars().all(|c| c == '.' || c == '^'),
            "unexpected char in {line:?}"
        );
        for s in line.chars().positions(|c| c == '^') {
            let enter = beams[s];
            if enter > 0 {
                beams[s - 1] += enter;
                beams[s] = 0;
                beams[s + 1] += enter;
            }
        }
    }
    beams.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
                                            "\
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
"};

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 21);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 40);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 1590);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 20571740188555);
    }
}
