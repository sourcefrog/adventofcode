use aoclib::Matrix;

fn main() {
    let input = input();
    println!("{}", solve1(&input));
    println!("{}", solve2(&input));
}

fn input() -> String {
    aoclib::input!()
}

fn solve1(input: &str) -> usize {
    let mut words = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();
    let ops = words.pop().unwrap();
    let mut total = 0;
    for col in 0..words[0].len() {
        let vals = words.iter().map(|l| l[col].parse::<usize>().unwrap());
        let col_result: usize = match ops[col] {
            "+" => vals.sum(),
            "*" => vals.product(),
            o => panic!("unknown op {o:?}"),
        };
        total += col_result;
    }
    total
}

fn solve2(input: &str) -> usize {
    let mat = Matrix::from_string_lines(input);
    let mut total: usize = 0;
    let mut accum: Vec<usize> = Vec::new(); // values to combine in current sum/prod
    for mut col in mat.columns().rev() {
        // Read digits down the column, accumulating them into `v`
        let &op = col.next_back().unwrap();
        let mut all_blank = true;
        let mut v = 0;
        for &ch in col {
            // dbg!(ch);
            if let Some(digit) = ch.to_digit(10) {
                v = v * 10 + digit as usize;
                all_blank = false;
            } else {
                assert_eq!(ch, ' ');
            }
        }
        if all_blank {
            assert_eq!(op, ' ');
        } else {
            accum.push(v);
            match op {
                '+' => total += accum.drain(..).sum::<usize>(),
                '*' => total += accum.drain(..).product::<usize>(),
                ' ' => (),
                other => panic!("unexpected {other:?}"),
            }
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = indoc::indoc! {
    "\
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
        "};

    #[test]
    fn example1() {
        assert_eq!(solve1(EXAMPLE), 4277556);
    }

    #[test]
    fn example2() {
        assert_eq!(solve2(EXAMPLE), 3263827);
    }

    #[test]
    fn solution1() {
        assert_eq!(solve1(&input()), 5335495999141);
    }

    #[test]
    fn solution2() {
        assert_eq!(solve2(&input()), 10142723156431);
    }
}
