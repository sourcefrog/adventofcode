// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/16

#![allow(unused_imports)]
use std::collections::BTreeMap;

use bitvec::prelude::*;

use itertools::Itertools;

use aoclib::{point, Matrix, Point};

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/16.txt").unwrap()
}

fn to_u32(bv: &BitSlice) -> u32 {
    bv.iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, b)| acc | ((*b as u32) << i))
}

struct Pkt {
    ver: u32,
    typ: PktType,
}

enum PktType {
    Literal,
    Operator(Vec<Pkt>),
}

impl Pkt {
    fn total_ver(&self) -> u32 {
        let mut tv: u32 = self.ver;
        if let PktType::Operator(subs) = &self.typ {
            tv += subs.iter().map(|s| s.total_ver()).sum::<u32>();
        }
        tv
    }
}

fn solve(input: &str) -> (u32, u32) {
    let sol_b = 0;

    let bits: BitVec = to_bits(input);
    let (_pos, pkts) = ppkts(&bits, u32::MAX);
    let sol_a = pkts.iter().map(|p| p.total_ver()).sum();
    (sol_a, sol_b)
}

fn ppkts(bits: &BitSlice, max_pkts: u32) -> (usize, Vec<Pkt>) {
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
    // Must be at least 6 bits to read another packet; the rest is padding.
    let ver = to_u32(&bits[pos..(pos + 3)]);
    pos += 3;
    let pktype = to_u32(&bits[pos..(pos + 3)]);
    pos += 3;
    println!("packet ver={ver} type={pktype}");
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
                typ: PktType::Literal,
            },
        );
    } else {
        // length type id
        let ltid = bits[pos];
        println!("  ltid={ltid}");
        pos += 1;
        if !ltid {
            let subpktlen = to_u32(&bits[pos..(pos + 15)]);
            println!("  subpktlen {subpktlen} {:?}", &bits[pos..(pos + 15)]);
            pos += 15;
            let (pq, subs) = ppkts(&bits[pos..(pos + subpktlen as usize)], u32::MAX);
            pos += pq;
            return (
                pos,
                Pkt {
                    ver,
                    typ: PktType::Operator(subs),
                },
            );
        } else {
            let nsubpkts = to_u32(&bits[pos..(pos + 11)]);
            pos += 11;
            let (pq, subs) = ppkts(&bits[pos..], nsubpkts);
            pos += pq;
            return (
                pos,
                Pkt {
                    ver,
                    typ: PktType::Operator(subs),
                },
            );
        }
    }
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
        let (pos, pkts) = ppkts(&bits, u32::MAX);
        assert_eq!(pkts.len(), 1);
        // assert_eq!(solve(EX).0, 1588);
    }

    #[test]
    fn example2() {
        let bits = to_bits("38006F45291200");
        let (_, pkt) = ppkt(&bits);
        if let PktType::Operator(subs) = pkt.typ {
            assert_eq!(subs.len(), 2);
        } else {
            panic!()
        };
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 652);
        assert_eq!(b, 2938);
    }
}
