#![feature(test)]

extern crate test;

use std::collections::HashMap;

type Polymer = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;
type Elements = HashMap<char, usize>;
struct Polymerization {
    polymer: Polymer,
    rules: Rules,
    elements: Elements,
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut p = Polymerization::new(input);
    for _ in 1..=10 {
        p.polymerize()
    }
    let part1_answer = p.diff_max_min_element();
    for _ in 11..=40 {
        p.polymerize()
    }
    let part2_answer = p.diff_max_min_element();
    (part1_answer, part2_answer)
}

impl Polymerization {
    fn new(input: &'static str) -> Polymerization {
        let input = input.trim_end().split('\n');
        let mut template = "";
        let mut rules: Rules = HashMap::new();

        for line in input {
            if template.is_empty() {
                template = line;
                continue;
            }
            if line.is_empty() {
                continue;
            }
            let c: Vec<char> = line.chars().collect();
            let k = (*c.get(0).unwrap(), *c.get(1).unwrap());
            let v = *c.get(6).unwrap();
            rules.insert(k, v);
        }

        let p: Vec<char> = template.chars().collect();
        let n = p.len();
        let mut polymer: Polymer = HashMap::new();
        for i in 0..n - 1 {
            *polymer.entry((p[i], p[i + 1])).or_default() += 1;
        }

        let elements: Elements = p.iter().fold(HashMap::new(), |mut m, e| {
            *m.entry(*e).or_default() += 1;
            m
        });

        Polymerization {
            polymer,
            rules,
            elements,
        }
    }

    fn polymerize(&mut self) {
        let mut new_polymer: Polymer = HashMap::new();
        for (k, v) in &self.polymer {
            let (a, b) = k;
            match self.rules.get(k) {
                Some(r) => {
                    *new_polymer.entry((*a, *r)).or_default() += v;
                    *new_polymer.entry((*r, *b)).or_default() += v;
                    *self.elements.entry(*r).or_default() += v;
                }
                None => {
                    *new_polymer.entry(*k).or_default() += v;
                }
            }
        }
        self.polymer = new_polymer
    }

    fn diff_max_min_element(&self) -> usize {
        let max = self.elements.values().max().unwrap();
        let min = self.elements.values().min().unwrap();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test1_parse() {
        let p = Polymerization::new(include_str!("../input-test1"));
        // "NNCB" -> ('N', 'N'), ('N', 'C'), ('C', 'B')
        assert_eq!(p.polymer.len(), 3);
        assert_eq!(*p.polymer.get(&('N', 'N')).unwrap(), 1);
        assert_eq!(*p.polymer.get(&('N', 'C')).unwrap(), 1);
        assert_eq!(*p.polymer.get(&('C', 'B')).unwrap(), 1);
        assert_eq!(p.rules.len(), 16);
    }

    #[test]
    fn test_input_test1_polymerize() {
        let mut p = Polymerization::new(include_str!("../input-test1"));
        p.polymerize();
        // "NCNBCHB" -> NC,CN,NB,BC,CH,HB
        assert_eq!(p.polymer.len(), 6);
        assert_eq!(*p.elements.get(&'N').unwrap(), 2);
        assert_eq!(*p.elements.get(&'C').unwrap(), 2);
        assert_eq!(*p.elements.get(&'H').unwrap(), 1);
        assert_eq!(*p.elements.get(&'B').unwrap(), 2);
        assert_eq!(p.elements.values().sum::<usize>(), 7);
        // step 2-5
        for _ in 2..=5 {
            p.polymerize();
        }
        assert_eq!(p.elements.values().sum::<usize>(), 97);
        // step 6-10
        for _ in 6..=10 {
            p.polymerize();
        }
        assert_eq!(p.elements.values().sum::<usize>(), 3073);
        // answer for part 1
        let part1_answer = p.diff_max_min_element();
        assert_eq!(part1_answer, 1588);
        // step 11-40
        for _ in 11..=40 {
            p.polymerize();
        }
        // answer for part 2
        let part2_answer = p.diff_max_min_element();
        assert_eq!(part2_answer, 2188189693529);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3697);
        assert_eq!(part2_answer, 4371307836157);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
