#![feature(test)]

extern crate test;

use std::collections::HashMap;

type Cave = usize;

#[derive(Default, Debug)]
struct Caves {
    caves: HashMap<&'static str, Cave>,
    small_caves: Vec<Cave>,
    next: Vec<Vec<Cave>>,
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

    let start = caves.start();
    let paths_part1 = caves.visit(start, Default::default(), false);
    let paths_part2 = caves.visit(start, Default::default(), true);
    (paths_part1, paths_part2)
}

impl Caves {
    pub fn new() -> Caves {
        Default::default()
    }

    fn add_next(&mut self, from: &'static str, to: &'static str) {
        if to != "start" && from != "end" {
            let u = self.cave_by_name(from);
            let v = self.cave_by_name(to);
            self.next.get_mut(u).unwrap().push(v);
        }
    }

    fn cave_by_name(&mut self, name: &'static str) -> Cave {
        match self.caves.get(name) {
            Some(cave) => *cave,
            None => {
                let cave = self.next.len() as Cave;
                self.caves.insert(name, cave);
                self.next.push(Vec::new());
                if name.chars().next().unwrap().is_lowercase() {
                    self.small_caves.push(cave);
                }
                cave
            }
        }
    }

    fn start(&self) -> Cave {
        *self.caves.get("start").unwrap()
    }

    fn is_small(&self, cave: Cave) -> bool {
        self.small_caves.contains(&cave)
    }

    fn visit(&self, cave: Cave, visited: &[Cave], double_allowed: bool) -> usize {
        let next_caves = self.next.get(cave).unwrap();
        if next_caves.is_empty() {
            return 1;
        }
        let mut paths = 0;
        for &next in next_caves.iter() {
            let mut double_allowed = double_allowed;
            if self.is_small(next) && visited.contains(&next) {
                if double_allowed {
                    double_allowed = false;
                } else {
                    continue;
                }
            }
            if self.is_small(cave) {
                let mut visited = visited.to_vec();
                visited.push(cave);
                paths += self.visit(next, &visited, double_allowed);
            } else {
                paths += self.visit(next, visited, double_allowed);
            }
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
