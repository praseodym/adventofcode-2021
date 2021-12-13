#![feature(test)]

extern crate test;

use std::fmt::Formatter;
use std::{cmp, fmt};

const N: usize = 1337; // x direction
const M: usize = 1337; // y direction
#[derive(Debug)]
struct Paper {
    dots: Box<[[bool; N]; M]>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq)]
struct Fold {
    direction: Direction,
    position: usize,
}
#[derive(Debug, PartialEq)]
enum Direction {
    X,
    Y,
}

fn main() {
    let (dots_part1, paper) = run(include_str!("../input"));
    println!("dots after one fold: {:?}", dots_part1);
    println!("paper after folds:\n{}", paper);
}

fn run(input: &'static str) -> (u16, Paper) {
    let (mut paper, folds) = parse_input(input);
    let mut dots_part1 = 0;
    for (i, fold) in folds.iter().enumerate() {
        let dots = paper.fold(fold);
        if i == 0 {
            dots_part1 = dots;
        }
    }
    (dots_part1, paper)
}

fn parse_input(input: &str) -> (Paper, Vec<Fold>) {
    let input = input.trim_end().split('\n');
    let mut paper: Paper = Paper::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut dots_parsed = false;

    for line in input {
        if line.is_empty() {
            // Start of folds
            dots_parsed = true;
            continue;
        }

        if !dots_parsed {
            let mut coords = line.split(',');
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            paper.dots[y][x] = true;
            paper.width = cmp::max(x, paper.width);
            paper.height = cmp::max(y, paper.height);
        } else {
            let fold = line.strip_prefix("fold along ").unwrap();
            let mut fold = fold.split('=');
            let direction = match fold.next().unwrap() {
                "x" => Direction::X,
                "y" => Direction::Y,
                _ => {
                    panic!("unknown fold direction")
                }
            };
            let position = fold.next().unwrap().parse().unwrap();
            let fold = Fold {
                direction,
                position,
            };
            folds.push(fold);
        }
    }
    (paper, folds)
}

impl Paper {
    pub fn new() -> Paper {
        Paper {
            dots: Box::new([[false; N]; M]),
            width: 0,
            height: 0,
        }
    }

    fn fold(&mut self, fold: &Fold) -> u16 {
        let mut dots = 0u16;
        match fold.direction {
            Direction::X => {
                self.width = fold.position - 1;
                for y in 0..=self.height {
                    for x in 0..=self.width {
                        let x2 = 2 * fold.position - x;
                        let dot = self.dots[y][x] || self.dots[y][x2];
                        if dot {
                            dots += 1;
                            self.dots[y][x] = dot;
                        }
                    }
                }
            }
            Direction::Y => {
                self.height = fold.position - 1;
                for y in 0..=self.height {
                    for x in 0..=self.width {
                        let y2 = 2 * fold.position - y;
                        let dot = self.dots[y][x] || self.dots[y2][x];
                        if dot {
                            dots += 1;
                            self.dots[y][x] = dot;
                        }
                    }
                }
            }
        }
        dots
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                write!(f, "{}", if self.dots[y][x] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test1_parse() {
        let (paper, folds) = parse_input(include_str!("../input-test1"));
        assert_eq!(paper.to_string(), include_str!("../input-test1-parsed"));
        assert_eq!(folds.len(), 2);
        assert_eq!(
            folds[0],
            Fold {
                direction: Direction::Y,
                position: 7
            }
        );
        assert_eq!(
            folds[1],
            Fold {
                direction: Direction::X,
                position: 5
            }
        );
    }

    #[test]
    fn test_input_test1_fold() {
        let (mut paper, folds) = parse_input(include_str!("../input-test1"));
        let dots = paper.fold(&folds[0]);
        println!("{}", paper);
        assert_eq!(dots, 17);
        assert_eq!(paper.to_string(), include_str!("../input-test1-folded1"));
        let dots = paper.fold(&folds[1]);
        println!("{}", paper);
        assert_eq!(dots, 16);
        assert_eq!(paper.to_string(), include_str!("../input-test1-folded2"));
    }

    #[test]
    fn test_input_own() {
        let (dots_part1, paper) = run(include_str!("../input"));
        assert_eq!(dots_part1, 747);
        assert_eq!(paper.to_string(), include_str!("../output"));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
