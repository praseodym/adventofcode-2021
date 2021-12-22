#![feature(test)]
#![feature(box_syntax)]

extern crate test;

use std::ops::Range;

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Step {
    action: bool,
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
    z1: isize,
    z2: isize,
}

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let steps = parse_input(input);
    let part1_answer = part1_run(&steps);
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<Step> {
    let mut ret = Vec::new();
    for line in input.trim_end().split('\n') {
        ret.push(parse_line(line));
    }
    ret
}

fn parse_line(line: &'static str) -> Step {
    let mut s = line.split(' ');
    let action = match s.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };

    let mut s = s.next().unwrap().split(',');
    let mut x = s.next().unwrap().strip_prefix("x=").unwrap().split("..");
    let mut y = s.next().unwrap().strip_prefix("y=").unwrap().split("..");
    let mut z = s.next().unwrap().strip_prefix("z=").unwrap().split("..");
    Step {
        action,
        x1: x.next().unwrap().parse().unwrap(),
        x2: x.next().unwrap().parse().unwrap(),
        y1: y.next().unwrap().parse().unwrap(),
        y2: y.next().unwrap().parse().unwrap(),
        z1: z.next().unwrap().parse().unwrap(),
        z2: z.next().unwrap().parse().unwrap(),
    }
}

fn part1_run(steps: &[Step]) -> usize {
    let mut state = vec![vec![vec![false; 101]; 101]; 101];
    for step in steps {
        for z in part1_range(step.z1, step.z2) {
            for y in part1_range(step.y1, step.y2) {
                for x in part1_range(step.x1, step.x2) {
                    state[z][y][x] = step.action;
                }
            }
        }
    }
    let mut ret = 0;
    for z in state {
        for y in z {
            for x in y {
                ret += x as usize;
            }
        }
    }
    ret
}

fn part1_range(a: isize, b: isize) -> Range<usize> {
    if (a < -50 && b < -50) || (a > 50 && b > 50) {
        return 0..0;
    }
    let mut a = a + 50;
    let mut b = b + 50;
    if a < 0 {
        a = 0;
    } else if a > 100 {
        a = 100;
    }
    if b < 0 {
        b = 0;
    } else if b > 100 {
        b = 100;
    } else {
        b += 1;
    }
    a as usize..b as usize
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_parse_example() {
        let steps = parse_input(include_str!("../input-example1"));
        assert_eq!(steps.len(), 4);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 39);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 590784);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 607657);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
