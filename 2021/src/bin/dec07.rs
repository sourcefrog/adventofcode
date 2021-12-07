// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/7

fn main() {
    let input = input();
    println!("{:?}", solve_a(&input));
    println!("{:?}", solve_b(&input));
}

fn input() -> Vec<i64> {
    std::fs::read_to_string("input/07.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|w| w.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn solve_a(ps: &[i64]) -> i64 {
    // Find the position that has the minimum total absolute
    // distance from all points.
    //
    // The simplest brute-force method is: consider every
    // point in the range between the maximum and minimum
    // number, calculate the distance to all other fish, and
    // we're done. However this seems pretty inefficient.
    //
    // Intuitively it seems like the best point might be the
    // median... but why?
    //
    // Also, the obvious way to find the median is to sort
    // and then pick the middle, but that's somewhat more
    // work than just looking at every point...
    //
    // If we knew it was the median and that the median
    // value was present in the input (rather than being the
    // mean of two points) then we could just look at all
    // points once...
    let mut best = i64::MAX;
    for i in 1..(*ps.iter().max().unwrap()) {
        let guess = ps.iter().map(|x| (x - i).abs()).sum();
        // We do know that there is a single minima; once
        // we've found it we're done.
        if guess <= best {
            best = guess
        } else {
            return best;
        }
    }
    unreachable!()
}

fn solve_b(ps: &[i64]) -> i64 {
    // Find the total distance from the best location using
    // a quadratic cost.
    //
    // This seems a lot like minimizing least-squares error,
    // or linear regression.
    //
    // In this case it seems there's less of a guarantee
    // that the goal is a data point in the input, and,
    // indeed, it seems that if we only consider
    // possibilities in the input then it doesn't find the
    // right answer.
    //
    // Possibly the `abs` in calculating the distance would
    // make it harder to state this as a polynomial?
    //
    // I wonder you could do something like Newton's method
    // to find the minimum?
    let mut best = i64::MAX;
    for i in 1..(*ps.iter().max().unwrap()) {
        let guess = ps
            .iter()
            .map(|x| {
                let d = (x - i).abs();
                (d * (d + 1)) / 2
            })
            .sum();
        // We do know that there is a single minima; once
        // we've found it we're done.
        if guess <= best {
            best = guess
        } else {
            return best;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let input = input();
        assert_eq!(solve_a(&input), 340056);
        assert_eq!(solve_b(&input), 96592275);
    }
}
