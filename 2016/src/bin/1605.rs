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

*/

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const DAY: &str = "1605";

fn solve_type_a(input: &str) -> String {
    let input = input.trim();
    (0..)
        .flat_map(|i| {
            let msg = format!("{}{}", input, i);
            let digest = md5::compute(msg.as_bytes());
            if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 {
                Some(char::from_digit((digest[2] & 0x0f) as u32, 16).unwrap())
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

/// Write a decimal representation of an integer into an existing byte buffer.
///
/// Return the number of bytes written if possible, or None if it does not fit.
#[must_use]
fn itoa(a: usize, buf: &mut [u8]) -> Option<usize> {
    let mut l = 0;
    // work out the length
    let mut aa = a;
    loop {
        l += 1;
        aa /= 10;
        if aa == 0 {
            break;
        }
    }
    if l > buf.len() {
        return None;
    }
    // now generate digits from right to left
    let mut aa = a;
    for i in (0..l).rev() {
        let d = (aa % 10) as u8;
        aa /= 10;
        buf[i] = b'0' + d;
    }
    Some(l)
}

fn solve_type_a_parallel(input: &str) -> String {
    let input = input.trim();
    let ibytes = input.as_bytes();
    let ilen = ibytes.len();
    let ncpus = num_cpus::get();
    const GOAL: usize = 8;
    // We need to find the first 8 hashes that have 5 leading zeroes.
    //
    // Start ncpus threads, each generating hashes.
    //
    // Each takes a number to generate from an atomic int, so we know they are
    // _started_ in order, but due to skew between the threads they may not
    // finish in order. Therefore we also need to remember the `i` that found a
    // match, and sort them when we're done.

    let iatomic = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    let found = AtomicUsize::new(0);
    crossbeam::scope(|scope| {
        for _ in 0..ncpus {
            scope.spawn(|_scope| {
                let mut buf = vec![0u8; ilen + 20];
                buf[..ilen].copy_from_slice(ibytes);
                loop {
                    let i = iatomic.fetch_add(1, Ordering::Relaxed);
                    // sleep(Duration::from_millis(1000));
                    let msglen = ilen + itoa(i, &mut buf[ilen..]).unwrap();
                    let digest = md5::compute(&buf[..msglen]);
                    if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0 {
                        let ch = char::from_digit((digest[2] & 0x0f) as u32, 16).unwrap();
                        let mut rlck = results.lock().unwrap();
                        rlck.push((i, ch));
                        if found.fetch_add(1, Ordering::AcqRel) >= GOAL {
                            break;
                        }
                    } else if found.load(Ordering::Acquire) >= GOAL {
                        break;
                    }
                }
            });
        }
    })
    .unwrap();

    let mut r = results.into_inner().unwrap();
    dbg!(r.len());
    assert_eq!(r.len(), GOAL);
    r.sort_unstable();
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

fn solve_a() -> String {
    solve_type_a(&input())
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
    fn solution_a() {
        assert_eq!(solve_a(), "c6697b55");
    }

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
    fn test_itoa(a: usize) {
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

    fn check_itoa(a: usize, buf: &mut [u8]) {
        let l: usize = itoa(a, buf).unwrap();
        assert_eq!(format!("{}", a).as_bytes(), &buf[..l]);
    }
}
