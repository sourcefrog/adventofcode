// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/19

#![allow(clippy::comparison_chain)] // bad warning; it's slower and no simpler
#![allow(unused_imports)]
use std::cmp::max;
use std::collections::HashSet;

use ndarray::prelude::*;

fn main() {
    let input = input();
    let (a, b) = solve(&input);
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/19.txt").unwrap()
}

type Pt = Array1<isize>;

fn solve(input: &str) -> (usize, isize) {
    let mut inp: Vec<Vec<Pt>> = Vec::new();
    for l in input.lines() {
        if l.starts_with("--- scanner ") {
            inp.push(Vec::new());
        } else if l.is_empty() {
        } else {
            let mut coo = l.split(',').map(|w| w.parse::<isize>().unwrap());
            let p = array![
                coo.next().unwrap(),
                coo.next().unwrap(),
                coo.next().unwrap(),
            ];
            inp.last_mut().unwrap().push(p);
        }
    }
    let inp = inp;
    let nscanners = inp.len();

    // Assume scanner 0 is at 0,0,0 with nominal orientation.
    // Treat its points as fixed.
    // Now try every rotation of every other matrix one by one until all of them are done.

    // [1, 0] [-1  0]  [ 0  -1]  [ 0  1 ]
    // [0, 1] [ 0 -1]  [ 1   0]  [ -1 0 ]
    let rot_x = arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
    let rot_y = arr2(&[[0, 0, 1], [0, 1, 0], [-1, 0, 0]]);
    let rot_z = arr2(&[[0, 1, 0], [-1, 0, 0], [0, 0, 1]]);
    let mut rots = Vec::new();
    for rx in 0..4 {
        for ry in 0..4 {
            for rz in 0..4 {
                let mut m = arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
                for _ in 0..rx {
                    m = m.dot(&rot_x);
                }
                for _ in 0..ry {
                    m = m.dot(&rot_y);
                }
                for _ in 0..rz {
                    m = m.dot(&rot_z);
                }
                if !rots.contains(&m) {
                    // println!("add {rx} {ry} {rz}");
                    rots.push(m);
                }
            }
        }
    }
    let nrots = rots.len();
    assert_eq!(rots.len(), 24);

    // First make a set of rotated points for each scanner, in each of the 24 possible
    // orientations.
    //
    // indexed[scanner][rotation][point]
    let mut rotpts: Vec<Vec<Vec<Pt>>> = Vec::new();
    for sc in &inp {
        let mut scrots: Vec<Vec<Pt>> = vec![vec![]; nrots];
        for (irot, rotm) in rots.iter().enumerate() {
            for p in sc {
                scrots[irot].push(p.dot(rotm));
            }
        }
        rotpts.push(scrots);
    }

    // scannerpos[iscanner] is the position of the scanner, if it's known.
    let mut scannerpos: Vec<Option<Array1<isize>>> = vec![None; nscanners];
    scannerpos[0] = Some(arr1(&[0, 0, 0]));
    // `fixed` is a set of all points whose absolute and final position is known.
    let mut fixed: HashSet<[isize; 3]> = HashSet::new();
    fixed.extend(inp[0].iter().map(toarr));
    // `fixedps[iscanner]` is a vec of the absolute positions of beacons for this scanner.
    let mut fixedpts: Vec<Vec<Pt>> = vec![vec![]; nscanners];
    fixedpts[0] = inp[0].clone();
    // `active` is a vec of scanner ids where we *have* found the location of the scanner,
    // but we *have not* yet found all its neighbors yet. In other words, these scanners
    // are the frontier that we can look to attach a new scanner to: or some of them
    // might find no attachments, and some might find more than one.
    let mut active: Vec<usize> = vec![0];
    // `unsolved` is a vec of scanner ids who we have not yet localized.
    let mut unsolved: Vec<usize> = (1..nscanners).collect();
    // TODO: Could probably use Rayon pretty well here, to run all candidates against the current
    // base scanner in parallel.
    while let Some(base) = active.pop() {
        debug_assert!(scannerpos[base].is_some());
        debug_assert!(!fixedpts[base].is_empty());
        for isc in unsolved.clone() {
            debug_assert!(scannerpos[isc].is_none());
            for irot in 0..nrots {
                // println!("try scanner {isc:2} rot {irot:2}");
                let roted: &[Pt] = &rotpts[isc][irot];
                if let Some(offset) = overlap(&fixedpts[base], roted) {
                    println!(
                        "** found overlap: scanner {isc:2} rot {irot:3} matched against scanner {base:2} offset {:?}",
                        offset.as_slice().unwrap()
                    );
                    fixedpts[isc] = roted.iter().map(|p| p - &offset).collect();
                    fixed.extend(roted.iter().map(|p| toarr(&(p - &offset))));
                    scannerpos[isc] = Some(offset);
                    unsolved.retain(|x| *x != isc);
                    active.push(isc);
                    break;
                    // println!("    fixed: {}", fixed.len());
                }
            }
        }
    }
    assert!(
        scannerpos.iter().all(|x| x.is_some()),
        "didn't locate all scanners",
    );

    let sol_a = fixed.len(); // number of beacons

    let mut sol_b = 0;
    let scannerpos: Vec<Pt> = scannerpos.into_iter().map(|x| x.unwrap()).collect();
    for a in &scannerpos {
        for b in &scannerpos {
            if a != b {
                let dist = (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs();
                // println!("{a:?} {b:?} {dist}");
                sol_b = max(sol_b, dist);
            }
        }
    }

    (sol_a, sol_b)
}

fn toarr(p: &Pt) -> [isize; 3] {
    [p[0], p[1], p[2]]
}

fn overlap(ap: &[Pt], bp: &[Pt]) -> Option<Pt> {
    // Consider every pair of points from a and b as a potential offset.
    // See how many other points in b match against a in with that offset.
    // If there are at least 12, that's a good match, return it.
    //
    // We don't need to look at offsets based on every point in bp: if there are
    // 12 points that match then we can skip the first 11 and still find at
    // least one anchor to align them.
    for b in &bp[11..] {
        for a in ap {
            let mut n = 1; // one is already force-aligned
            let off = b - a;
            for c in bp {
                if c == b {
                    break;
                }
                let d = c - &off;
                if ap.contains(&d) {
                    n += 1;
                    if n >= 12 {
                        // println!("found match {off:?} n={n}");
                        return Some(off);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn example() {
        let (a, b) = solve(EX);
        assert_eq!(a, 79);
        assert_eq!(b, 3621);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 405);
        assert_eq!(b, 12306);
    }
}
