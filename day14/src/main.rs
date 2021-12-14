#![feature(test)]
#![feature(linked_list_cursors)]

extern crate test;

use std::collections::{HashMap, LinkedList};

type Rules = HashMap<(char, char), char>;

fn main() {
    let part1_answer = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
}

fn run(input: &'static str) -> usize {
    let (template, rules) = parse_input(input);
    let mut new_polymer = template.to_string();
    for _ in 1..=10 {
        new_polymer = polymerize(&new_polymer, &rules);
    }
    part1_count(&new_polymer)
}

fn parse_input(input: &'static str) -> (&str, Rules) {
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
        // TODO: char indexing instead of split?
        let mut s = line.split(" -> ");
        let mut k = s.next().unwrap().chars();
        let k = (k.next().unwrap(), k.next().unwrap());
        let v = s.next().unwrap().chars().next().unwrap();
        rules.insert(k, v);
    }

    (template, rules)
}

fn polymerize(polymer: &str, rules: &Rules) -> String {
    let mut p: LinkedList<char> = LinkedList::new();
    polymer.chars().for_each(|c| p.push_back(c));
    let l = p.len();
    let mut pc = p.cursor_front_mut();

    for _ in 1..l {
        pc.move_next();
        let a = *pc.peek_prev().unwrap();
        let b = *pc.current().unwrap();
        // println!("pair: {}{}", a, b);
        if let Some(r) = rules.get(&(a, b)) {
            // println!("insert {} at position {}", r, i);
            pc.insert_before(*r);
        }
    }
    // TODO: return LinkedList instead?
    let mut ret = String::new();
    p.iter().for_each(|c| ret.push(*c));
    ret
}

fn part1_count(polymer: &str) -> usize {
    let counts = polymer
        .chars()
        .into_iter()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            *m.entry(c).or_default() += 1;
            m
        });
    let max = counts.iter().map(|(_, v)| *v).max().unwrap();
    let min = counts.iter().map(|(_, v)| *v).min().unwrap();
    max - min
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test1_parse() {
        let (template, rules) = parse_input(include_str!("../input-test1"));
        assert_eq!(template, "NNCB");
        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_input_test1_polymerize() {
        let (template, rules) = parse_input(include_str!("../input-test1"));
        // step 1
        let new_polymer = polymerize(template, &rules);
        assert_eq!(new_polymer, "NCNBCHB");
        // step 2
        let new_polymer = polymerize(&new_polymer, &rules);
        assert_eq!(new_polymer, "NBCCNBBBCBHCB");
        // step 3
        let new_polymer = polymerize(&new_polymer, &rules);
        assert_eq!(new_polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        // step 4
        let new_polymer = polymerize(&new_polymer, &rules);
        assert_eq!(
            new_polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
        // step 5-10
        let mut new_polymer = new_polymer;
        for _ in 5..=10 {
            new_polymer = polymerize(&new_polymer, &rules);
        }
        assert_eq!(new_polymer.len(), 3073);
        // answer for part 1
        let part1_answer = part1_count(&new_polymer);
        assert_eq!(part1_answer, 1588);
    }

    #[test]
    fn test_input_own() {
        let part1_answer = run(include_str!("../input"));
        // assert_eq!(part1_answer, 747);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
