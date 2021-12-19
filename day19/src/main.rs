#![feature(test)]
#![feature(drain_filter)]

extern crate test;

use std::collections::{HashMap, HashSet};

type Coordinate = (isize, isize, isize);

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut scanners = parse_input(input);
    let part1_answer = reduce(&mut scanners);
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<Vec<Coordinate>> {
    let mut ret = Vec::new();
    let mut scanner = Vec::new();
    for line in input.trim_end().split('\n') {
        if line.is_empty() {
            continue;
        }
        if line.contains("scanner") {
            if !scanner.is_empty() {
                ret.push(scanner);
                scanner = Vec::new();
            }
            continue;
        }
        let mut s = line.split(',').map(|n| n.parse().unwrap());
        scanner.push((s.next().unwrap(), s.next().unwrap(), s.next().unwrap()));
    }
    ret.push(scanner);
    ret
}

fn rotate(beacon: &Coordinate, rotation: u8) -> Coordinate {
    let (x, y, z) = *beacon;
    match rotation {
        0 => (x, y, z),
        1 => (x, -y, -z),
        2 => (x, z, -y),
        3 => (x, -z, y),
        4 => (-x, -y, z),
        5 => (-x, y, -z),
        6 => (-x, -z, -y),
        7 => (-x, z, y),
        8 => (y, x, -z),
        9 => (y, -x, z),
        10 => (y, z, x),
        11 => (y, -z, -x),
        12 => (-y, x, z),
        13 => (-y, -x, -z),
        14 => (-y, z, -x),
        15 => (-y, -z, x),
        16 => (z, x, y),
        17 => (z, -x, -y),
        18 => (z, -y, x),
        19 => (z, y, -x),
        20 => (-z, x, -y),
        21 => (-z, -x, y),
        22 => (-z, -y, -x),
        23 => (-z, y, x),
        _ => unreachable!("invalid rotation"),
    }
}

fn rotate_all(beacons: &[Coordinate], rotation: u8) -> Vec<Coordinate> {
    beacons.iter().map(|b| rotate(b, rotation)).collect()
}

fn reduce(scanners: &mut Vec<Vec<Coordinate>>) -> usize {
    let mut base: HashSet<Coordinate> = HashSet::from_iter(scanners.remove(0).iter().cloned());
    while !scanners.is_empty() {
        let mut progress = false;
        scanners.drain_filter(|scanner| {
            for r in 0..24 {
                let rotated = rotate_all(scanner, r);
                let mut distances: HashMap<Coordinate, usize> = HashMap::new();
                for b in &rotated {
                    for s in &base {
                        let distance = (s.0 - b.0, s.1 - b.1, s.2 - b.2);
                        *distances.entry(distance).or_default() += 1;
                    }
                }
                let &max = distances.values().max().unwrap();
                if max >= 12 {
                    let d = distances.iter().find(|(_, &v)| v == max).unwrap().0;
                    for s in &rotated {
                        base.insert((s.0 + d.0, s.1 + d.1, s.2 + d.2));
                    }
                    progress = true;
                    return true;
                }
            }
            false
        });
        assert!(progress, "made no progress in reduction phase");
    }
    base.len()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_example1() {
        let scanners = parse_input(include_str!("../input-example1"));
        assert_eq!(scanners.len(), 5);
    }

    #[test]
    fn test_example2() {
        let mut scanners = parse_input(include_str!("../input-example2"));
        assert_eq!(scanners.len(), 5);
        let n = reduce(&mut scanners);
        assert_eq!(n, 79);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 335);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
