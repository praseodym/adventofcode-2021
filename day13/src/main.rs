#![feature(test)]
#![feature(type_alias_impl_trait)]

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
    let dots_part1 = run(include_str!("../input"));
    println!("dots after one fold: {:?}", dots_part1);
}

fn run(input: &'static str) -> u16 {
    let (paper, folds) = parse_input(input);
    let (_, dots) = fold(&paper, &folds[0]);
    dots
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

fn fold(paper: &Paper, fold: &Fold) -> (Paper, u16) {
    let mut folded_paper: Paper = Paper::new();
    let mut dots = 0u16;
    match fold.direction {
        Direction::X => {
            folded_paper.width = fold.position - 1;
            folded_paper.height = paper.height;
            for y in 0..=folded_paper.height {
                for x in 0..=folded_paper.width {
                    let x2 = 2 * fold.position - x;
                    let dot = paper.dots[y][x] || paper.dots[y][x2];
                    if dot {
                        dots += 1;
                        folded_paper.dots[y][x] = dot;
                    }
                }
            }
        }
        Direction::Y => {
            folded_paper.width = paper.width;
            folded_paper.height = fold.position - 1;
            for y in 0..=folded_paper.height {
                for x in 0..=folded_paper.width {
                    let y2 = 2 * fold.position - y;
                    let dot = paper.dots[y][x] || paper.dots[y2][x];
                    if dot {
                        dots += 1;
                        folded_paper.dots[y][x] = dot;
                    }
                }
            }
        }
    }
    (folded_paper, dots)
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
        let (paper, folds) = parse_input(include_str!("../input-test1"));
        let (folded_paper, dots) = fold(&paper, &folds[0]);
        println!("{}", folded_paper);
        assert_eq!(dots, 17);
        assert_eq!(
            folded_paper.to_string(),
            include_str!("../input-test1-folded1")
        );
        let (folded_paper, dots) = fold(&folded_paper, &folds[1]);
        println!("{}", folded_paper);
        assert_eq!(dots, 16);
        assert_eq!(
            folded_paper.to_string(),
            include_str!("../input-test1-folded2")
        );
    }

    #[test]
    fn test_input_own() {
        let dots_part1 = run(include_str!("../input"));
        assert_eq!(dots_part1, 747);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
