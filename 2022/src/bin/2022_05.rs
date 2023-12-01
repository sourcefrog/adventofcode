//! https://adventofcode.com/2022/day/5

fn main() {
    println!("{}", solve_a(&input()));
    println!("{}", solve_b(&input()));
}

fn input() -> String {
    std::fs::read_to_string("input/05.txt").unwrap()
}

struct Instr {
    cnt: usize,
    from_stack: usize,
    to_stack: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Instr>) {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut lines = input.lines();
    for l in lines.by_ref() {
        if l.starts_with(" 1 ") {
            break;
        }
        for (i, s) in stacks.iter_mut().enumerate() {
            let c = l.chars().nth(1 + 4 * i).unwrap();
            assert!(c.is_ascii_uppercase() || c == ' ');
            if c.is_ascii_uppercase() {
                s.insert(0, c);
            }
        }
    }
    assert!(lines.next().unwrap().trim().is_empty());
    let mut instructions = Vec::new();
    for l in lines {
        let w: Vec<&str> = l.split_ascii_whitespace().collect();
        let cnt: usize = w[1].parse().unwrap();
        let from_stack: usize = w[3].parse::<usize>().unwrap() - 1;
        let to_stack: usize = w[5].parse::<usize>().unwrap() - 1;
        instructions.push(Instr {
            cnt,
            from_stack,
            to_stack,
        });
    }
    (stacks, instructions)
}

fn solve_a(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    for Instr {
        cnt,
        from_stack,
        to_stack,
    } in instructions
    {
        for _ in 0..cnt {
            let x = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(x);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn solve_b(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    for Instr {
        cnt,
        from_stack,
        to_stack,
    } in instructions
    {
        let j = stacks[from_stack].len() - cnt;
        let mut x = stacks[from_stack].split_off(j);
        assert_eq!(x.len(), cnt);
        stacks[to_stack].append(&mut x);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_a() {
        assert_eq!(solve_a(&input()), "VPCDMSLWJ");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(&input()), "TPWCGNCCG");
    }
}
