// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/16

use bitvec::prelude::*;

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

fn to_u64(bv: &BitSlice) -> u64 {
    // dbg!(bv.len());
    assert!(bv.len() <= 64);
    bv.iter()
        .rev()
        .enumerate()
        .filter(|(_, b)| **b)
        .fold(0u64, |acc, (i, _)| acc | (1 << i))
}

struct Pkt {
    ver: u64,
    typ: PktType,
}

enum PktType {
    Literal(u64),
    Operator(u64, Vec<Pkt>),
}

impl Pkt {
    fn total_ver(&self) -> u64 {
        let mut tv: u64 = self.ver;
        if let PktType::Operator(_, subs) = &self.typ {
            tv += subs.iter().map(|s| s.total_ver()).sum::<u64>();
        }
        tv
    }

    fn eval(&self) -> u64 {
        use PktType::*;
        match &self.typ {
            Literal(x) => *x,
            Operator(ot, subs) => {
                let mut svs = subs.iter().map(|s| s.eval());
                match ot {
                    0 => svs.sum(),
                    1 => svs.product(),
                    2 => svs.min().unwrap(),
                    3 => svs.max().unwrap(),
                    5 => (svs.next().unwrap() > svs.next().unwrap()) as u64,
                    6 => (svs.next().unwrap() < svs.next().unwrap()) as u64,
                    7 => (svs.next().unwrap() == svs.next().unwrap()) as u64,
                    _ => panic!("bad op {}", ot),
                }
            }
        }
    }
}

fn solve(input: &str) -> (u64, u64) {
    let bits: BitVec = to_bits(input);
    let (_pos, pkts) = ppkts(&bits, 1);
    let sol_a = pkts.iter().map(|p| p.total_ver()).sum();
    let sol_b = pkts[0].eval();
    (sol_a, sol_b)
}

fn ppkts(bits: &BitSlice, max_pkts: u64) -> (usize, Vec<Pkt>) {
    let mut pos = 0;
    let mut pkts = Vec::new();
    while pos < (bits.len() - 6) && pkts.len() < max_pkts as usize {
        let (taken, pkt) = ppkt(&bits[pos..]);
        pos += taken;
        pkts.push(pkt)
    }
    (pos, pkts)
}

fn ppkt(bits: &BitSlice) -> (usize, Pkt) {
    let mut pos = 0;
    let ver = to_u64(&bits[pos..(pos + 3)]);
    pos += 3;
    let pktype = to_u64(&bits[pos..(pos + 3)]);
    pos += 3;
    // println!("packet ver={ver} type={pktype}");
    if pktype == 4 {
        let mut literal = bitvec![];
        loop {
            let cont = bits[pos];
            literal.extend_from_bitslice(&bits[(pos + 1)..(pos + 5)]);
            pos += 5;
            if !cont {
                break;
            }
        }
        return (
            pos,
            Pkt {
                ver,
                typ: PktType::Literal(to_u64(&literal)),
            },
        );
    }
    // length type id
    let ltid = bits[pos];
    // println!("  ltid={ltid}");
    pos += 1;
    let subs;
    if !ltid {
        let subpktlen = to_u64(&bits[pos..(pos + 15)]);
        // println!("  subpktlen {subpktlen} {:?}", &bits[pos..(pos + 15)]);
        pos += 15;
        let (pq, s) = ppkts(&bits[pos..(pos + subpktlen as usize)], u64::MAX);
        subs = s;
        pos += pq;
    } else {
        let nsubpkts = to_u64(&bits[pos..(pos + 11)]);
        pos += 11;
        let (pq, s) = ppkts(&bits[pos..], nsubpkts);
        subs = s;
        pos += pq;
    }
    (
        pos,
        Pkt {
            ver,
            typ: PktType::Operator(pktype, subs),
        },
    )
}

fn to_bits(input: &str) -> BitVec {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .flat_map(|c| (0..4).rev().map(move |i| c & (1 << i) != 0))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let bits = to_bits("D2FE28");
        assert_eq!(
            bits.iter()
                .map(|b| if *b { '1' } else { '0' })
                .collect::<String>(),
            "110100101111111000101000"
        );
        let (_pos, pkts) = ppkts(&bits, u64::MAX);
        assert_eq!(pkts.len(), 1);
        // assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn example2() {
        let bits = to_bits("38006F45291200");
        let (_, pkt) = ppkt(&bits);
        if let PktType::Operator(_, subs) = pkt.typ {
            assert_eq!(subs.len(), 2);
        } else {
            panic!()
        };
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 945);
        assert_eq!(b, 10637009915279);
    }
}
