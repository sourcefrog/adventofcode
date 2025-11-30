use aoclib::input;

fn main() {
    let input = &input();
    println!("{}", solve_one(input));
    println!("{}", solve_two(input));
}

fn input() -> String {
    input!()
}

fn hash(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |h, c| ((h + *c as usize) * 17) & 0xff)
}

fn solve_one(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn solve_two(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for step in input.trim().split(',') {
        if let Some(label) = step.strip_suffix('-') {
            boxes[hash(label)].retain(|(l, _fl)| *l != label);
        } else {
            let len = step.len();
            let label = &step[..len - 2];
            assert_eq!(step.as_bytes()[len - 2], b'=');
            let fl = (step.as_bytes()[len - 1] as char)
                .to_digit(10)
                .expect("not a digit") as usize;
            if let Some(x) = boxes[hash(label)].iter_mut().find(|(l, _)| *l == label) {
                x.1 = fl;
            } else {
                boxes[hash(label)].push((label, fl))
            }
        }
    }
    focus_power(&boxes)
}

fn focus_power(boxes: &Vec<Vec<(&str, usize)>>) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(ibox, abox)| {
            abox.iter()
                .enumerate()
                .map(move |(ilens, (_label, fl))| (1 + ibox) * (1 + ilens) * fl)
        })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            solve_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            solve_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }

    #[test]
    fn solution_1() {
        assert_eq!(solve_one(&input()), 517551);
    }

    #[test]
    fn solution_2() {
        assert_eq!(solve_two(&input()), 286097);
    }
}
