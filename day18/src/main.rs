use itertools::Itertools;

type Number = Vec<(usize, usize)>;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let numbers = parse_input(input);
    let part1_answer = magnitude(&sum(&numbers));
    let part2_answer = max_magnitude(numbers);
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Vec<Number> {
    let mut ret = Vec::new();
    let input = input.trim_end().split('\n');
    for line in input {
        ret.push(parse_line(line));
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

fn sum(numbers: &[Number]) -> Number {
    let mut res = numbers[0].clone();
    numbers.iter().skip(1).for_each(|x| add(&mut res, x));
    res
}

fn add(a: &mut Number, b: &Number) {
    a.append(&mut b.clone());
    a.iter_mut().for_each(|x| x.1 += 1);
    reduce(a);
}

fn reduce(a: &mut Number) {
    while explode(a) || split(a) {}
}

fn explode(number: &mut Number) -> bool {
    let len = number.len();
    for i in 0..len {
        if i + 1 < len {
            let (a, v) = number[i];
            let (b, w) = number[i + 1];
            if v == 5 && w == 5 {
                if i != 0 {
                    number[i - 1].0 += a;
                }
                if len > i + 2 {
                    number[i + 2].0 += b;
                }
                number.drain(i..i + 2);
                number.insert(i, (0, 4));
                return true;
            }
        }
    }
    false
}

fn split(number: &mut Number) -> bool {
    for (i, (a, n)) in number.clone().iter().enumerate() {
        if *a >= 10 {
            let half = *a / 2;
            let remainder = *a % 2;
            number[i] = (half + remainder, n + 1);
            number.insert(i, (half, n + 1));
            return true;
        }
    }
    false
}

fn magnitude(number: &Number) -> usize {
    let mut n = number.clone();
    for depth in (1..=4).rev() {
        'depth: loop {
            let len = n.len();
            for i in 0..len {
                if i + 1 < len {
                    let (a, v) = n[i];
                    let (b, w) = n[i + 1];
                    if v == depth && w == depth {
                        n[i] = (3 * a + 2 * b, depth - 1);
                        n.remove(i + 1);
                        continue 'depth;
                    }
                }
            }
            break;
        }
    }
    n[0].0
}

fn max_magnitude(numbers: Vec<Number>) -> usize {
    numbers
        .into_iter()
        .permutations(2)
        .map(|v| magnitude(&sum(&v)))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
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
        add(a, &parse_line("[1,1]"));
        reduce(a);
        assert_eq!(*a, parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_reduce() {
        let a = &mut parse_line("[1,2]");
        add(a, &parse_line("[[3,4],5]"));
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
        let v = parse_input(include_str!("../input-test1"));
        let n = sum(&v);
        assert_eq!(
            n,
            parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_example2() {
        let v = parse_input(include_str!("../input-test2"));
        let n = sum(&v);
        assert_eq!(
            n,
            parse_line("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(magnitude(&n), 4140);
        assert_eq!(max_magnitude(v), 3993);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3935);
        assert_eq!(part2_answer, 4669);
    }
}
