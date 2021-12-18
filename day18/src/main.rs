#![feature(test)]
#![feature(array_windows)]
#![feature(int_roundings)]

extern crate test;

use std::collections::VecDeque;

type Number = Vec<(usize, usize)>;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut numbers = parse_input(input);
    let part1_answer = magnitude(&sum(&mut numbers));
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> VecDeque<Number> {
    let mut ret = VecDeque::new();
    let input = input.trim_end().split('\n');
    for line in input {
        ret.push_back(parse_line(line));
    }
    ret
}

fn parse_line(line: &str) -> Number {
    let mut res: Number = Vec::new();
    let mut depth = 0;
    for c in line.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            _ => res.push((c.to_digit(10).unwrap() as usize, depth)),
        }
    }
    res
}

fn sum(numbers: &mut VecDeque<Number>) -> Number {
    let mut res = numbers.pop_front().unwrap();
    numbers.iter_mut().for_each(|x| add(&mut res, x));
    res
}

fn add(a: &mut Number, b: &mut Number) {
    a.append(b);
    a.iter_mut().for_each(|x| x.1 += 1);
    reduce(a);
}

fn reduce(a: &mut Number) {
    while explode(a) || split(a) {}
}

fn explode(number: &mut Number) -> bool {
    for (i, &[(a, v), (b, w)]) in number.clone().array_windows().enumerate() {
        if v == 5 && w == 5 {
            if i != 0 {
                number.get_mut(i - 1).unwrap().0 += a;
            }
            if number.len() > i + 2 {
                number.get_mut(i + 2).unwrap().0 += b;
            }
            number.drain(i..i + 2);
            number.insert(i, (0, 4));
            return true;
        }
    }
    false
}

fn split(number: &mut Number) -> bool {
    for (i, (a, n)) in number.clone().iter().enumerate() {
        if *a >= 10 {
            number[i] = (a.unstable_div_ceil(2), n + 1);
            number.insert(i, (a.unstable_div_floor(2), n + 1));
            return true;
        }
    }
    false
}

fn magnitude(number: &Number) -> usize {
    let mut n = number.clone();
    for depth in (1..=4).rev() {
        'depth: loop {
            for (i, &[(a, v), (b, w)]) in n.clone().array_windows().enumerate() {
                if v == depth && w == depth {
                    n[i] = (3 * a + 2 * b, depth - 1);
                    n.remove(i + 1);
                    continue 'depth;
                }
            }
            break;
        }
    }
    n[0].0
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = "[9,[8,7]]";
        let n = parse_line(input);
        assert_eq!(n, vec![(9, 1), (8, 2), (7, 2)])
    }

    #[test]
    fn test_explode() {
        let testcases = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];
        for t in testcases {
            let mut n = parse_line(t.0);
            let r = explode(&mut n);
            assert!(r);
            assert_eq!(n, parse_line(t.1));
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_split() {
        let testcases = vec![
            (
                vec![(0, 4), (7, 4), (4, 3), (15, 3), (0, 4), (13, 4), (1, 2), (1, 2)],
                vec![(0, 4), (7, 4), (4, 3), (7, 4), (8, 4), (0, 4), (13, 4), (1, 2), (1, 2)],
            ),
            (
                vec![(0, 4), (7, 4), (4, 3), (7, 4), (8, 4), (0, 4), (13, 4), (1, 2), (1, 2)],
                parse_line("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"),
            ),
        ];
        for t in testcases {
            let mut n = t.0.clone();
            let r = split(&mut n);
            assert!(r);
            assert_eq!(n, t.1);
        }
    }

    #[test]
    fn test_add() {
        let a = &mut parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        add(a, &mut parse_line("[1,1]"));
        reduce(a);
        assert_eq!(*a, parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_reduce() {
        let a = &mut parse_line("[1,2]");
        add(a, &mut parse_line("[[3,4],5]"));
        assert_eq!(*a, parse_line("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_magnitude() {
        let testcases = vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for t in testcases {
            assert_eq!(magnitude(&parse_line(t.0)), t.1);
        }
    }

    #[test]
    fn test_example1() {
        let mut v = parse_input(include_str!("../input-test1"));
        let n = sum(&mut v);
        assert_eq!(
            n,
            parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_example2() {
        let mut v = parse_input(include_str!("../input-test2"));
        let n = sum(&mut v);
        assert_eq!(
            n,
            parse_line("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(magnitude(&n), 4140);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3935);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
