use aoclib::Matrix;

static PUZZLE: &str = env!("CARGO_BIN_NAME");

fn main() {
    let input = &input();
    println!("{PUZZLE} a {}", solve_one(input));
    println!("{PUZZLE} b {}", solve_two(input));
}

fn input() -> String {
    aoclib::input!()
}

fn split_inputs(input: &str) -> Vec<Matrix<bool>> {
    let mut matrixes = Vec::new();
    for block in input.split("\n\n") {
        let map = Matrix::from_string_lines(block).map(|c| *c == '#');
        matrixes.push(map)
    }
    matrixes
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RowOrCol {
    Row(usize),
    Col(usize),
}

impl RowOrCol {
    fn summary(&self) -> usize {
        match self {
            RowOrCol::Row(x) => 100 * x,
            RowOrCol::Col(x) => *x,
        }
    }
}

fn solve_one(input: &str) -> usize {
    split_inputs(input)
        .into_iter()
        .map(|map| find_one_reflection(&map).summary())
        .sum()
}

fn find_one_reflection(map: &Matrix<bool>) -> RowOrCol {
    let refls = find_reflections(map);
    assert_eq!(refls.len(), 1);
    refls[0]
}

fn find_reflections(map: &Matrix<bool>) -> Vec<RowOrCol> {
    let h = map.height();
    let w = map.width();
    let mut res = Vec::new();

    'col: for xmirror in 1..w {
        // See if columns 0..xmirror are a reflection of xmirror..w.
        for x1 in (2 * xmirror).saturating_sub(w)..xmirror {
            let x2 = (xmirror - x1) + xmirror - 1;
            assert!(x2 < w);
            // println!("{xmirror}: check col {x1} against {x2}");
            if !map.column(x1).eq(map.column(x2)) {
                // println!("   >mismatch");
                continue 'col;
            }
        }
        // println!("found reflection at x={xmirror}");
        res.push(RowOrCol::Col(xmirror));
    }

    'row: for ymirror in 1..h {
        for y in (2 * ymirror).saturating_sub(h)..ymirror {
            let y2 = (ymirror - y) + (ymirror - 1);
            debug_assert!(y2 < h);
            // println!("{ymirror}: check {y} against {y2}");
            if !map.row(y).eq(map.row(y2)) {
                continue 'row;
            }
        }
        // println!("found reflection at y={ymirror}");
        res.push(RowOrCol::Row(ymirror));
    }
    res
}

fn solve_two(input: &str) -> usize {
    let mut sum = 0;
    'map: for map in split_inputs(input) {
        let orig = find_one_reflection(&map);

        // Try inverting all points until we find one that generates a different reflection
        for (smudge_point, val) in map.point_values() {
            let mut altered = map.clone();
            altered[smudge_point] = !val;

            let mut altered_refl = find_reflections(&altered);
            altered_refl.retain(|rc| *rc != orig);
            if altered_refl.len() == 1 {
                // println!("found smudge point {smudge_point:?}");
                sum += altered_refl[0].summary();
                continue 'map;
            }
        }
        unreachable!("didn't find a smudge point");
    }
    sum
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"\
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        " };

    #[test]
    fn example_1() {
        assert_eq!(solve_one(EXAMPLE), 405);
    }

    #[test]
    fn solution_one() {
        assert_eq!(solve_one(&input()), 27505);
    }

    #[test]
    fn example_two() {
        assert_eq!(solve_two(EXAMPLE), 400);
    }

    #[test]
    fn solution_two() {
        assert_eq!(solve_two(&input()), 22906);
    }
}
