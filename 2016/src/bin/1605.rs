//! https://adventofcode.com/2016/day/5

/*
Benchmarks for part A:

Compute naively and sequentially on a single thread until 8 results are found:
    Benchmark #1: /home/mbp/tmp/1605a-serial
     Time (mean ± σ):     953.4 ms ±  10.4 ms    [User: 952.9 ms, System: 0.4 ms]
     Range (min … max):   938.4 ms … 973.6 ms    10 runs

Run ncpus threads until enough results are found (f2a5ee2d):

    Benchmark #1: ../target/release/1605
      Time (mean ± σ):     142.4 ms ±  52.8 ms    [User: 3.372 s, System: 0.003 s]
      Range (min … max):    90.8 ms … 259.7 ms    50 runs

(It's noticeably noisy.)

Using only physical cores with `num_cpus::get_physical` is slower:

    Benchmark #1: ../target/release/1605a
    Time (mean ± σ):     211.4 ms ±  88.3 ms    [User: 2.443 s, System: 0.001 s]
    Range (min … max):   130.7 ms … 352.4 ms    50 runs

Checking for changes less frequently, is much better: presumably less ping-ponging
of the variable across threads (9a021acba4b319cfb9d14636d14e4a001da0c78a):

    > hyperfine ../target/release/1605 --warmup 20 -m 50
    Benchmark #1: ../target/release/1605
    Time (mean ± σ):      91.4 ms ±   2.2 ms    [User: 2.158 s, System: 0.001 s]
    Range (min … max):    86.5 ms …  95.0 ms    50 runs

This suggests perhaps we could do without sharing i: what if each thread walked
through the series with less coordination? This is much better again
(9e7d3e39bc258cb06db2e44a93af2f4e10eadb83):

    Benchmark #1: ../target/release/1605
    Time (mean ± σ):      49.2 ms ±   3.4 ms    [User: 1.107 s, System: 0.001 s]
    Range (min … max):    47.2 ms …  74.2 ms    62 runs

Switching from a std::sync::Mutex to a RwLock makes things incrementally better again
(c58b82d3e36a359aa0f0c1179c9fbc67c245f848):

    Benchmark #1: ../target/release/1605
    Time (mean ± σ):      42.5 ms ±   2.6 ms    [User: 964.6 ms, System: 1.4 ms]
    Range (min … max):    40.5 ms …  55.5 ms    100 runs

Generating digits in reverse order should do fewer integer divisions but makes no
perceptible difference:

    Benchmark #1: ../target/release/1605
    Time (mean ± σ):      42.7 ms ±   3.1 ms    [User: 969.3 ms, System: 1.0 ms]
    Range (min … max):    40.6 ms …  62.3 ms    100 runs


*/

use parking_lot::RwLock;

const DAY: &str = "1605";

/// Write a decimal representation of an integer into an existing byte buffer.
///
/// Return the number of bytes written if possible, or None if it does not fit.
#[must_use]
fn itoa(a: u64, buf: &mut [u8]) -> Option<usize> {
    let mut l = 0;
    let mut revdigs = [0u8; 22];
    // work out the length and accumulate digits in reverse order.
    let mut aa = a;
    loop {
        revdigs[l] = b'0' + (aa % 10) as u8;
        aa /= 10;
        l += 1;
        if aa == 0 {
            break;
        }
    }
    if l > buf.len() {
        return None;
    }
    for i in 0..l {
        buf[l - i - 1] = revdigs[i];
    }
    Some(l)
}

fn solve_type_a_parallel(input: &str) -> String {
    // We need to find the first 8 hashes that have 5 leading zeroes.
    //
    // Start ncpus threads, each generating hashes.
    //
    // They will not necessarily find matches in the right order.
    let ibytes = input.trim().as_bytes();
    let ilen = ibytes.len();
    let ncpus = num_cpus::get();
    const GOAL: usize = 8;
    let rrw = RwLock::new(Vec::with_capacity(GOAL * 2));
    crossbeam::scope(|scope| {
        // shadow external variables so that they're not moved/copied.
        // https://stackoverflow.com/a/58459786/243712
        let rrw = &rrw;
        for thread_i in 0..ncpus {
            scope.spawn(move |_scope| {
                let mut buf = vec![0u8; ilen + 20];
                buf[..ilen].copy_from_slice(ibytes);
                for j in 0.. {
                    let i = j * ncpus + thread_i;
                    let msglen = ilen + itoa(i as u64, &mut buf[ilen..]).unwrap();
                    let digest = md5::compute(&buf[..msglen]);
                    if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 {
                        let ch = char::from_digit((digest[2] & 0x0f) as u32, 16).unwrap();
                        rrw.write().push((i, ch));
                        // println!("found at i={}", i);
                    }
                    // If there are GOAL elements all with index <i then this
                    // thread cannot possibly find any more good answers.
                    else if j % 256 == 0 {
                        let r = rrw.read();
                        if r.len() >= GOAL && r.iter().all(|(ii, _c)| *ii < i) {
                            // println!("{} is done", thread_i);
                            break;
                        }
                    }
                }
            });
        }
    })
    .unwrap();

    let mut r = rrw.into_inner();
    dbg!(r.len());
    // assert_eq!(r.len(), GOAL);
    r.sort_unstable();
    dbg!(&r);
    r.iter().take(GOAL).map(|(_i, ch)| ch).collect()
}

fn solve_type_b(input: &str) -> String {
    let input = input.trim();
    let mut r = ['-'; 8];
    for i in 0.. {
        let msg = format!("{}{}", input, i);
        let digest = md5::compute(msg.as_bytes());
        if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 {
            let pos = (digest[2] & 0x0f) as usize;
            if pos > 7 || r[pos] != '-' {
                continue;
            }
            r[pos] = char::from_digit((digest[3] >> 4) as u32, 16).unwrap();
            println!("{}", r.iter().collect::<String>());
            if !r.contains(&'-') {
                return r.iter().collect();
            }
        }
    }
    unreachable!()
}

fn input() -> String {
    std::fs::read_to_string(&format!("input/{}.txt", DAY)).unwrap()
}

fn solve_b() -> String {
    solve_type_b(&input())
}

fn main() {
    println!("{}a: {}", DAY, solve_type_a_parallel(&input()));
    // println!("{}a: {}", DAY, solve_a());
    // println!("{}b: {}", DAY, solve_b());
}

#[cfg(test)]
mod test1605 {
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn solution_a_parallel() {
        assert_eq!(solve_type_a_parallel(&input()), "c6697b55");
    }

    #[test]
    fn solution_b() {
        assert_eq!(solve_b(), "8c35d1ab");
    }

    proptest! {
    #[test]
    fn test_itoa(a: u64) {
        println!("a={}", a);
        let mut buf = [0u8; 30];
        check_itoa(a, &mut buf);
    }}

    #[test]
    fn test_itoa_basics() {
        check_itoa(0, &mut [0u8]);
        check_itoa(1, &mut [0u8]);
        check_itoa(2, &mut [0u8]);
    }

    #[test]
    fn itoa_too_small() {
        assert!(itoa(100, &mut []).is_none());
        assert!(itoa(100, &mut [0]).is_none());
        assert!(itoa(100, &mut [0, 0]).is_none());
        assert!(itoa(0, &mut []).is_none());
    }

    fn check_itoa(a: u64, buf: &mut [u8]) {
        let l: usize = itoa(a, buf).unwrap();
        assert_eq!(format!("{}", a).as_bytes(), &buf[..l]);
    }
}
