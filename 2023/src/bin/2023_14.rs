use aoclib::{Dir, Matrix, input};

fn main() {
    let input = &input!();
    println!("{}", solve_one(input));
    println!("{}", solve_two(input));
}

fn solve_one(input: &str) -> usize {
    let mut map = Matrix::from_string_lines(input);
    slide(&mut map, Dir::N);
    // println!("{}", map.to_string_lines());
    calc_load(&map)
}

fn solve_two(input: &str) -> usize {
    let mut map = Matrix::from_string_lines(input);
    static MANY_CYCLES: usize = 1000000000;
    // To calculate this many cycles let's try to find a recurring pattern in the map:
    // once we find that meta-cycle we should be able to extrapolate where it will end up.
    let mut history = Vec::new();
    for i in 0..300 {
        slide(&mut map, Dir::N);
        slide(&mut map, Dir::W);
        slide(&mut map, Dir::S);
        slide(&mut map, Dir::E);
        if let Some(recur) = history.iter().enumerate().rev().find(|(_i, v)| **v == map) {
            let prev_i = recur.0;
            // println!("found recurrence from {prev_i} to {i}");
            let metacycle_len = i - prev_i;
            let end_on = ((MANY_CYCLES - prev_i - 1) % metacycle_len) + prev_i;
            return calc_load(&history[end_on]);
        } else {
            history.push(map.clone());
            // println!("cycle {i}:\n{}", map.to_string_lines());
        }
    }
    unreachable!()
}

/// Slide all round rocks in one direction as far as they will go.
fn slide(map: &mut Matrix<char>, dir: Dir) {
    loop {
        let mut any_moved = false;
        for p in map.points() {
            if map[p] == 'O' {
                let p2 = p.step(dir);
                if map.try_get(p2) == Some(&'.') {
                    map[p2] = 'O';
                    map[p] = '.';
                    any_moved = true;
                }
            }
        }
        if !any_moved {
            break;
        }
    }
}

fn calc_load(map: &Matrix<char>) -> usize {
    let h = map.height();
    map.point_values()
        .filter(|(_p, v)| **v == 'O')
        .map(|(p, _v)| h - TryInto::<usize>::try_into(p.y).unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{input, solve_one, solve_two};

    static EXAMPLE: &str = indoc! { "\
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        "};

    #[test]
    fn example_one() {
        let sol = solve_one(EXAMPLE);
        assert_eq!(sol, 136);
    }

    #[test]
    fn solution_one() {
        assert_eq!(solve_one(&input!()), 105003);
    }

    #[test]
    fn example_two() {
        assert_eq!(solve_two(EXAMPLE), 64);
    }

    #[test]
    fn solution_two() {
        assert_eq!(solve_two(&input!()), 93742);
    }
}
