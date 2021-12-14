#![feature(test)]
#![feature(linked_list_cursors)]

extern crate test;

use std::collections::{HashMap, LinkedList};

type Polymer = LinkedList<char>;
type Rules = HashMap<(char, char), char>;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let (mut polymer, rules) = parse_input(input);
    for _ in 1..=10 {
        polymerize(&mut polymer, &rules);
    }
    let part1_answer = answer_count(&polymer);
    // for _ in 11..=40 {
    //     polymerize(&mut polymer, &rules);
    // }
    // let part2_answer = answer_count(&new_polymer);
    // (part1_answer, part2_answer)
    (part1_answer, 0)
}

fn parse_input(input: &'static str) -> (Polymer, Rules) {
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

    let template = polymer_to_linkedlist(template);

    (template, rules)
}

fn polymerize<'a>(polymer: &'a mut Polymer, rules: &'a Rules) -> &'a mut Polymer {
    let l = polymer.len();
    let mut pc = polymer.cursor_front_mut();

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
    polymer
}

fn polymer_to_linkedlist(polymer: &str) -> Polymer {
    let mut p: LinkedList<char> = LinkedList::new();
    polymer.chars().for_each(|c| p.push_back(c));
    p
}

fn polymer_to_string(polymer: &Polymer) -> String {
    let mut ret = String::new();
    polymer.iter().for_each(|c| ret.push(*c));
    ret
}

fn answer_count(polymer: &Polymer) -> usize {
    let counts = polymer
        .into_iter()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            *m.entry(*c).or_default() += 1;
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
        assert_eq!(polymer_to_string(&template), "NNCB");
        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_input_test1_polymerize() {
        let (mut polymer, rules) = parse_input(include_str!("../input-test1"));
        // step 1
        polymerize(&mut polymer, &rules);
        assert_eq!(polymer_to_string(&polymer), "NCNBCHB");
        // step 2
        let polymer = polymerize(&mut polymer, &rules);
        assert_eq!(polymer_to_string(polymer), "NBCCNBBBCBHCB");
        // step 3
        polymerize(polymer, &rules);
        assert_eq!(polymer_to_string(polymer), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        // step 4
        polymerize(polymer, &rules);
        assert_eq!(
            polymer_to_string(polymer),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
        // step 5-10
        for _ in 5..=10 {
            polymerize(polymer, &rules);
        }
        assert_eq!(polymer.len(), 3073);
        // answer for part 1
        let part1_answer = answer_count(polymer);
        assert_eq!(part1_answer, 1588);

        // // step 11-40
        // for _ in 11..=40 {
        //     polymerize(&mut polymer, &rules);
        // }
        // // answer for part 2
        // let part2_answer = answer_count(&polymer);
        // assert_eq!(part2_answer, 2188189693529);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3697);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
