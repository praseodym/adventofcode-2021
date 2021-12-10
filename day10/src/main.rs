use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');

    let mut part1 = 0u32;
    let mut part2_scores: Vec<u64> = Vec::new();

    'outer: for line in input {
        let mut s: Vec<char> = Vec::new();
        for char in line.chars() {
            match char {
                '(' => s.push(char),
                '[' => s.push(char),
                '{' => s.push(char),
                '<' => s.push(char),
                ')' => {
                    if s.pop().unwrap() != '(' {
                        part1 += 3;
                        continue 'outer;
                    }
                }
                ']' => {
                    if s.pop().unwrap() != '[' {
                        part1 += 57;
                        continue 'outer;
                    }
                }
                '}' => {
                    if s.pop().unwrap() != '{' {
                        part1 += 1197;
                        continue 'outer;
                    }
                }
                '>' => {
                    if s.pop().unwrap() != '<' {
                        part1 += 25137;
                        continue 'outer;
                    }
                }
                _ => {
                    panic!("invalid character {}", char);
                }
            }
        }
        let mut part2_score = 0u64;
        while !s.is_empty() {
            let char = s.pop().unwrap();
            part2_score *= 5;
            part2_score += match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => {
                    panic!("invalid character {}", char);
                }
            };
        }
        part2_scores.push(part2_score);
    }

    part2_scores.sort_unstable();
    let &part2 = part2_scores.get(part2_scores.len() / 2).unwrap();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    assert_eq!(part1, 166191);
    assert_eq!(part2, 1152088313);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}
