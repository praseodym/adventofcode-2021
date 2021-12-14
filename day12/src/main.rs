#![feature(test)]

extern crate test;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Cave {
    name: &'static str,
    small: bool,
}

#[derive(Default, Debug)]
struct Caves {
    cache: HashMap<&'static str, Rc<Cave>>,
    next: HashMap<Rc<Cave>, HashSet<Rc<Cave>>>,
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
        caves.load(u, v);
    });

    let paths_part1 = caves.visit(caves.get_start(), Default::default(), false);
    let paths_part2 = caves.visit(caves.get_start(), Default::default(), true);
    (paths_part1, paths_part2)
}

impl Cave {
    pub fn new(name: &'static str) -> Cave {
        Cave {
            name,
            small: name.chars().next().unwrap().is_lowercase(),
        }
    }
}

impl Caves {
    pub fn new() -> Caves {
        Default::default()
    }

    fn get(&mut self, name: &'static str) -> Rc<Cave> {
        if self.cache.get(name).is_none() {
            self.cache.insert(name, Rc::new(Cave::new(name)));
        }
        self.cache.get(name).unwrap().clone()
    }

    pub fn load(&mut self, from: &'static str, to: &'static str) {
        let u = self.get(from);
        let v = self.get(to);
        self.add_next(u.clone(), v.clone());
        self.add_next(v, u);
    }

    fn add_next(&mut self, from: Rc<Cave>, to: Rc<Cave>) {
        if to.name != "start" && from.name != "end" {
            self.next.entry(from).or_default().insert(to);
        }
    }

    fn get_start(&self) -> Rc<Cave> {
        self.cache.get("start").unwrap().clone()
    }

    fn visit(&self, cave: Rc<Cave>, visited: &[Rc<Cave>], double_allowed: bool) -> usize {
        let mut paths = 0;
        let mut visited = visited.to_vec();
        if cave.small {
            visited.push(cave.clone());
        }
        if let Some(next_caves) = self.next.get(&cave) {
            for next in next_caves.iter() {
                let mut double_allowed = double_allowed;
                if next.small && visited.contains(next) {
                    if double_allowed {
                        double_allowed = false;
                    } else {
                        continue;
                    }
                }
                paths += self.visit(next.clone(), &visited, double_allowed);
            }
        } else {
            return 1;
        }
        paths
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Cave {}
impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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
