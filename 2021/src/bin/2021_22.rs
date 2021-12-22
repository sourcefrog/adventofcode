// Copyright 2021 Martin Pool

//! https://adventofcode.com/2021/day/22 "Reactor Reboot"

use regex;

use ndarray::prelude::*;

fn main() {
    let (a, b) = solve(&input());
    println!("{}", a);
    println!("{}", b);
}

fn input() -> String {
    std::fs::read_to_string("input/22.txt").unwrap()
}

fn solve(input: &str) -> (usize, u64) {
    let re = regex::Regex::new(
        r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)",
    )
    .unwrap();
    let mut cubes: Array3<bool> = Array3::default((101, 101, 101));
    let mut insts: Vec<(bool, [isize; 6])> = Vec::new();
    for l in input.lines() {
        let caps = re.captures(l).unwrap();
        let on = caps[1] == *"on";
        let v: Vec<isize> = caps
            .iter()
            .skip(2)
            .map(|p| p.unwrap().as_str().parse::<isize>().unwrap())
            .collect();
        if let [x0, x1, y0, y1, z0, z1] = v[..6] {
            insts.push((on, [x0, x1, y0, y1, z0, z1]));
            if x0 < -50 || x1 > 50 || y0 < -50 || y1 > 50 || z0 < -50 || z1 > 50 {
                continue;
            }
            dbg!(on, &v);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    for z in z0..=z1 {
                        cubes[[(x + 50) as usize, (y + 50) as usize, (z + 50) as usize]] = on;
                    }
                }
            }
        }
    }
    let sol_a = cubes.iter().filter(|x| **x).count();

    // So we only need to worry about the coordinates on each axis that
    // are actually mentioned in the input: the values can never vary in
    // between.
    //
    // Let's make a map from each x, y, z value to a simpler integer index
    // of the unique values on that axis. We can then make a simpler cuboid
    // with one-bit cells between each of those values. The volume of each
    // cuboid depends on its real coordinates. We can just walk them to
    // collect the total volume.
    //
    // (It's still kinda big at 580M elements.)

    // Collect the unique x values.
    let mut xvals: Vec<isize> = Vec::new();
    let mut yvals = Vec::new();
    let mut zvals = Vec::new();
    for (_on, [x0, x1, y0, y1, z0, z1]) in &insts {
        xvals.push(*x0);
        xvals.push(*x1 + 1);
        yvals.push(*y0);
        yvals.push(*y1 + 1);
        zvals.push(*z0);
        zvals.push(*z1 + 1);
    }
    xvals.sort();
    xvals.dedup();
    yvals.sort();
    yvals.dedup();
    zvals.sort();
    zvals.dedup();
    dbg!(xvals.len(), yvals.len(), zvals.len());

    // lit[xi,yi,zi] == true means that the voxel of xvals[xi]..xvals[xi+1] etc
    // is lit.
    //
    // Ranges are inclusive so the natural representation is also
    // inclusive. But we need to be able to handle the case where an off
    // overlaps by one with an `on` and so it turns off one column.
    //
    // So we'll track them as semi-open, and convert to that form when
    // generating the array.
    let mut lit: Array3<bool> =
        Array3::default((xvals.len() + 1, yvals.len() + 1, zvals.len() + 1));
    dbg!(lit.len());
    for (on, [x0, x1, y0, y1, z0, z1]) in insts {
        let xi0 = idx(x0, &xvals);
        let xi1 = idx(x1 + 1, &xvals);
        let yi0 = idx(y0, &yvals);
        let yi1 = idx(y1 + 1, &yvals);
        let zi0 = idx(z0, &zvals);
        let zi1 = idx(z1 + 1, &zvals);
        for i in xi0..xi1 {
            for j in yi0..yi1 {
                for k in zi0..zi1 {
                    lit[[i, j, k]] = on;
                }
            }
        }
    }

    let mut sol_b: u64 = 0;
    for ((xi, yi, zi), &on) in lit.indexed_iter() {
        if on {
            sol_b += ((xvals[xi + 1] - xvals[xi])
                * (yvals[yi + 1] - yvals[yi])
                * (zvals[zi + 1] - zvals[zi])) as u64;
        }
    }

    (sol_a, sol_b)
}

fn idx(x: isize, vals: &[isize]) -> usize {
    vals.iter().position(|y| *y == x).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex_b() {
        let ex = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
";
        let (_, b) = solve(ex);
        assert_eq!(b, 2758514936282235);
    }

    #[test]
    fn solution() {
        let (a, b) = solve(&input());
        assert_eq!(a, 648681);
        assert_eq!(b, 1302784472088899);
    }
}
