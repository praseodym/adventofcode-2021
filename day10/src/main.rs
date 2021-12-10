use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');

    let mut part1 = 0u32;

    for line in input {
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
                        break;
                    }
                }
                ']' => {
                    if s.pop().unwrap() != '[' {
                        part1 += 57;
                        break;
                    }
                }
                '}' => {
                    if s.pop().unwrap() != '{' {
                        part1 += 1197;
                        break;
                    }
                }
                '>' => {
                    if s.pop().unwrap() != '<' {
                        part1 += 25137;
                        break;
                    }
                }
                _ => {
                    println!("invalid character {}", char)
                }
            }
        }
    }

    println!("part 1: {}", part1);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}
