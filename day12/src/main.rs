#![feature(test)]

extern crate test;

use std::collections::{HashMap, HashSet};

type Cave = &'static str;

#[derive(Default, Debug)]
struct Caves {
    next: HashMap<Cave, HashSet<Cave>>,
}

fn main() {
    let (paths_part1, paths_part2) = run(include_str!("../input"));

    println!("paths part 1: {}", paths_part1);
    println!("paths part 2: {}", paths_part2);
}

fn run(input: &'static str) -> (usize, usize) {
    let input = input.trim_end().split('\n');
    let mut caves = Caves::new();
    input.map(|l| l.split('-')).for_each(|mut s| {
        let u = s.next().unwrap();
        let v = s.next().unwrap();
        caves.add_next(u, v);
        caves.add_next(v, u);
    });

    let paths_part1 = caves.visit("start", Default::default(), false);
    let paths_part2 = caves.visit("start", Default::default(), true);
    (paths_part1, paths_part2)
}

fn is_small(cave: Cave) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

impl Caves {
    pub fn new() -> Caves {
        Default::default()
    }

    fn add_next(&mut self, from: Cave, to: Cave) {
        if to != "start" && from != "end" {
            self.next.entry(from).or_default().insert(to);
        }
    }

    fn visit(&self, cave: Cave, visited: &[Cave], double_allowed: bool) -> usize {
        let mut paths = 0;
        let mut visited = visited.to_vec();
        if is_small(cave) {
            visited.push(cave);
        }
        if let Some(next_caves) = self.next.get(&cave) {
            for next in next_caves.iter() {
                let mut double_allowed = double_allowed;
                if is_small(next) && visited.contains(next) {
                    if double_allowed {
                        double_allowed = false;
                    } else {
                        continue;
                    }
                }
                paths += self.visit(next, &visited, double_allowed);
            }
        } else {
            return 1;
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test1() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test1"));
        assert_eq!(paths_part1, 10);
        assert_eq!(paths_part2, 36);
    }

    #[test]
    fn test_input_test2() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test2"));
        assert_eq!(paths_part1, 19);
        assert_eq!(paths_part2, 103);
    }

    #[test]
    fn test_input_test3() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test3"));
        assert_eq!(paths_part1, 226);
        assert_eq!(paths_part2, 3509);
    }

    #[test]
    fn test_input_own() {
        let (paths_part1, paths_part2) = run(include_str!("../input"));
        assert_eq!(paths_part1, 5958);
        assert_eq!(paths_part2, 150426);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
