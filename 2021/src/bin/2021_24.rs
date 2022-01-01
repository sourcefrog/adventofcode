// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/24

fn main() {
    let (a, b) = solve(&input());
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/24.txt").unwrap()
}

type Regs = [isize; 4];

fn regidx(name: &str) -> usize {
    let c = name.chars().next().unwrap();
    assert!(('w'..='z').contains(&c), "invalid reg {:?}", name);
    (c as u32 - 'w' as u32) as usize
}

fn run(program: &[&str], regs: &mut Regs, input: &mut Vec<isize>) {
    for l in program {
        // println!("{}", l);
        let w: Vec<&str> = l.split_whitespace().collect();
        if w[0] == "inp" {
            regs[regidx(w[1])] = input.remove(0)
        } else {
            let a = regidx(w[1]);
            let bname = w[2];
            let bval: isize = bname
                .parse::<isize>()
                .unwrap_or_else(|_| regs[regidx(bname)]);
            match w[0] {
                "mul" => regs[a] *= bval,
                "add" => regs[a] += bval,
                "div" => regs[a] /= bval,
                "mod" => regs[a] %= bval,
                "eql" => regs[a] = (regs[a] == bval) as isize,
                _ => panic!("unimplemented {:?}", l),
            }
        }
    }
}

fn solve(input: &str) -> (usize, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let chunks: Vec<_> = lines.chunks(18).collect();
    for (i, c) in chunks.iter().enumerate() {
        println!("** chunk {i}");
        assert_eq!(c[0], "inp w");
        assert_eq!(c[1], "mul x 0");
        assert_eq!(c[2], "add x z");
        assert_eq!(c[3], "mod x 26");
        let div_z: isize = c[4].strip_prefix("div z ").unwrap().parse().unwrap();
        let add_x: isize = c[5].strip_prefix("add x ").unwrap().parse().unwrap();
        assert_eq!(c[6], "eql x w");
        assert_eq!(c[7], "eql x 0");
        assert_eq!(c[8], "mul y 0");
        assert_eq!(c[9], "add y 25");
        assert_eq!(c[10], "mul y x");
        assert_eq!(c[11], "add y 1");
        assert_eq!(c[12], "mul z y");
        assert_eq!(c[13], "mul y 0");
        assert_eq!(c[14], "add y w");
        let add_y: isize = c[15].strip_prefix("add y ").unwrap().parse().unwrap();
        assert_eq!(c[16], "mul y x");
        assert_eq!(c[17], "add z y");

        dbg!(div_z, add_x, add_y);
        // for input_digit in (1..=9).rev() {
        //     let regs = run(&c, [0; 4], &mut vec![input_digit]);
        //     println!("input digit {} -> {:?}", input_digit, regs);
        // }
        // break;

        // x = z % 26 + add_x
        // z /= div_z
        // x = (x != input)
        // y = if x 26 else 1
        // z *= y
        // y = input + add_y
        // if x { z += y }
        //
        // To keep z at 0 we want x = 0 so that z is multiplied by 1 and then 0 is added.
        // Is it just all 9s?
        //
        // We can't keep it all at 0 all the way along, because the
        // available inputs aren't enough to match the necessary values.
        //
        // But could we work out what values will produce an 0 from the last
        // module? We only carry across state in z. So we need to find an
        // input and a penultimate z that give the right result?
        //
        // So we can choose any z for the second-last round that generates
        // the maximum input. But... eventually z has to be 0 at the start.
        //
        // Can we do some kind of DFS for valid inputs? It seems like that
        // would converge on visiting all the numbers, which is infeasible.
        //
        // Interestingly div_z is always 1 or 26. And add_y is always positive,
        // while add_x may be negative. And there are div_z = 26 or a reduction
        // in 7 of the rounds, exactly half.
        //
        // or restating
        //
        // x = (z % 26 + add_x) != input
        // z /= div_z
        // if x {
        //   z *= 26
        //   z += input + add_y
        // }
        //
        // Because input and add_y are always positive, how can we end up with
        // z=0 in the final round? It seems like only if x=0. If x were !0,
        // if z was negative it would be too negative to come back to 0, since input+add_y are less
        // than 26. If z is positive or zero then it will end up positive.
        //
        // So we need a prev_z and add_x such that they add up to the input.
        // Add_x is -9; so input digit 9 works if prev_z is 18.
        //
        // Generally it's constrained that (z[12] % 26 - 9) == input[12].
        //
        // Hm what about going forward from 0, what possible z values do
        // we generate?
        //
        // Round 0, x will always be true because add_x is 11. z ends up at (input+add_y) which
        // is input+15 so in the range 15..23. As a base-26 number we can
        // look at z as [input[0]+14].
        //
        // Round 1, given z in that range, and add_x of 14. x is also always
        // true. div_z is 1 so has no effect. So z is multiplied by 26,
        // we add the input, and then add_y of 6. So z ends up over a big
        // range from 397 to 613, but perhaps it's still OK because it'll be reduced in the next
        // round?
        // So z is then [input[0]+14, input[1]+6].
        //
        // Chunk 2, we look at (z % 26) + add_x, which from the previous round
        // is input[1] + add_y[1] + add_x[2]. They're all significantly
        // positive so x is definetly true. z is not divided. We push another
        // base-26 digit onto z which is now
        // [input[0]+14, input[1]+6, input[2]+6].
        //
        // Chunk 3, again add_x is 13 so x has to be true. We don't divide
        // z but rather push another digit:
        // [input[0]+14, input[1]+6, input[2]+6, input[3]+13].
        //
        // (So we can kind of look at the use of mul/div/mod 26 as being like
        // a stack within z.
        //
        // Chunk 4, maybe more interesting: add_x is -12, the last digit of z is
        // input[3] + 13, so x *can* be false if input[4] = input[3] + 1?
        // Would we *want* it to be false? It seems like it makes things
        // simpler later? In fact it seems like if we don't take the
        // opportunity to pop off z when we can, it won't get back to 0.
        // So, let's say that input[4] must be input[3]+1 and then after
        // this the rightmost digit of z is dropped leaving it at
        // [input[0]+14, input[1]+6, input[2]+6].
    }
    assert_eq!(chunks.len(), 14);

    let mut regs = [0; 4];
    for chunk in &chunks[..4] {
        run(chunk, &mut regs, &mut vec![7]);
    }
    assert_eq!(regs[3], mkbase26(&[7 + 14, 7 + 6, 7 + 6, 7 + 13]));

    // Theory that chunk[4] pops
    let regs = run_chunks(&chunks[..=4], &[8, 8, 8, 8, 9]);
    assert_eq!(regs[1], 0);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6, 8 + 6]));

    // Chunk 5, add_x is +10, x is always true, don't divide, push
    // input[5] + 8 so z is
    // [input[0]+14, input[1]+6, input[2]+6, input[5] + 8]

    // Chunk 6, add_x is -15 so x can be false if input[6] = (input[5]+ 8 - 15)
    // i.e. input[6] = input[5] - 7. Then we can pop one. Leaving.
    // [input[0]+14, input[1]+6, input[2]+6]
    let regs = run_chunks(&chunks[..=6], &[8, 8, 8, 8, 9, 9, 2]);
    assert_eq!(regs[1], 0);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6, 8 + 6]));
    // Chunk 7, halfway there, also push
    // [input[0]+14, input[1]+6, input[2]+6, input[7] + 10]

    // z: [input[0]+14, input[1]+6, input[2]+6, input[7] + 10, input[8] + 8]
    println!("Chunk 8, also push.");
    let regs = run_chunks(&chunks[..=8], &[8, 8, 8, 8, 9, 9, 2, 9, 9]);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6, 8 + 6, 9 + 10, 9 + 8]));

    // Chunk 9, pop with add_x=-13, so input[9] = input[8] + 8 - 13, i.e.
    // input[8] - 5.
    // z: [input[0]+14, input[1]+6, input[2]+6, input[7] + 10]
    println!("Chunk 9, pop");
    let regs = run_chunks(&chunks[..=9], &[8, 8, 8, 8, 9, 9, 2, 9, 9, 4]);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6, 8 + 6, 9 + 10]));

    // Chunk 10, pop with add_x = -13, so input[10] = input[7] + 10 - 13.
    // z: [input[0]+14, input[1]+6, input[2]+6]
    println!("Chunk 10, pop");
    let regs = run_chunks(&chunks[..=10], &[8, 8, 8, 8, 9, 9, 2, 9, 9, 4, 6]);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6, 8 + 6,]));

    // I randomly added 8s to start with, but here we need to subtract 14 so
    // the right answer for input[3] must be 9, and input[11] can be 1.
    println!("Chunk 11, pop");
    let regs = run_chunks(&chunks[..=11], &[8, 8, 9, 8, 9, 9, 2, 9, 9, 4, 6, 1]);
    assert_eq!(regs[3], mkbase26(&[8 + 14, 8 + 6,]));

    // The input is input[1] + 6 - 2 so the input1[1] must be 5 so that this can be 9.
    println!("Chunk 12, pop");
    let regs = run_chunks(&chunks[..=12], &[9, 5, 9, 8, 9, 9, 2, 9, 9, 4, 6, 1, 9]);
    assert_eq!(regs[3], mkbase26(&[9 + 14,]));

    // input[0] + 14 - 9, so input[0] = 4 and input[13] = 9
    println!("Chunk 13, pop");
    let regs = run_chunks(&chunks[..=13], &[4, 5, 9, 8, 9, 9, 2, 9, 9, 4, 6, 1, 9, 9]);
    assert_eq!(regs[3], mkbase26(&[]));

    println!("part b");
    let regs = run_chunks(&chunks[..=13], &[1, 1, 9, 8, 9, 9, 2, 9, 9, 4, 6, 1, 5, 6]);
    assert_eq!(regs[3], mkbase26(&[]));

    let regs = run_chunks(&chunks[..=13], &[1, 1, 9, 8, 9, 9, 2, 4, 9, 4, 1, 1, 5, 6]);
    assert_eq!(regs[3], mkbase26(&[]));

    let regs = run_chunks(&chunks[..=13], &[1, 1, 9, 8, 9, 9, 2, 4, 6, 1, 1, 1, 5, 6]);
    assert_eq!(regs[3], mkbase26(&[]));

    let regs = run_chunks(&chunks[..=13], &[1, 1, 9, 1, 2, 9, 2, 4, 6, 1, 1, 1, 5, 6]);
    assert_eq!(regs[3], mkbase26(&[]));

    let regs = run_chunks(&chunks[..=13], &[1, 1, 9, 1, 2, 8, 1, 4, 6, 1, 1, 1, 5, 6]);
    assert_eq!(regs[3], mkbase26(&[]));

    // for input in 1..=9 {
    //     let regs = run(&chunks[0], [0; 4], &mut vec![input]);
    //     println!("round 0 input {input} => {}", regs[3]);
    // }

    // println!("round 1");
    // for z in 15..=23 {
    //     for input in 1..=9 {
    //         let regs = run(&chunks[1], [0, 0, 0, z], &mut vec![input]);
    //         print!("{:4} ", regs[3]);
    //     }
    //     println!();
    // }

    // let mut input = vec![9; 14];
    // let regs = run(&lines, [0; 4], &mut input);
    // dbg!(&regs);

    let sol_a = 0;
    let sol_b = 0;

    (sol_a, sol_b)
}

fn run_chunks(chunks: &[&[&str]], input: &[isize]) -> Regs {
    assert_eq!(chunks.len(), input.len());
    let mut regs = [0; 4];
    let mut input: Vec<isize> = input.into();
    for ch in chunks {
        run(ch, &mut regs, &mut input);
    }
    assert!(input.is_empty());
    dbg!(&regs);
    regs
}

fn mkbase26(a: &[isize]) -> isize {
    let mut r = 0;
    for x in a {
        r *= 26;
        r += x;
    }
    r
}

// #[derive(Debug)]
// struct St {
//     digits: Vec<isize>,
//     z: isize,
// }

// // Given a program and the list of acceptable z outputs, return a list of all
// // predecessor z values and the inputs that would work with them.
// fn one_round(program: &[&str], goal_sts: &[St]) -> Vec<St> {
//     let mut r = Vec::new();
//     'input: for input in (1..=9).rev() {
//         for st in goal_sts {
//             for prev_z in 0..26 {
//                 let regs = run(&program, [0, 0, 0, prev_z], &mut vec![input]);
//                 if regs[3] == st.z {
//                     // println!("input digit {}, prev_z = {prev_z} -> {:?}", input, regs);
//                     let mut digits = vec![input];
//                     digits.extend_from_slice(&st.digits);
//                     r.push(St { digits, z: prev_z });
//                 }
//             }
//         }
//     }
//     r
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 0);
        assert_eq!(b, 0);
    }
}
