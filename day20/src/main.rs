#![feature(test)]

extern crate test;

use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;

const M: usize = 54;
const N: usize = 100 + 2 * M;

#[derive(Debug)]
struct Image {
    enhancement: [bool; 512],
    image: [[bool; N]; N],
    margin: usize,
    size: usize,
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut image = parse_input(input);
    image = image.enhance_image().enhance_image();
    let part1_answer = image.count_lit();
    for _ in 3..=50 {
        image = image.enhance_image();
    }
    let part2_answer = image.count_lit();
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> Image {
    let mut input = input.trim_end().split('\n');

    let mut enhancement = [false; 512];
    input
        .next()
        .unwrap()
        .chars()
        .map(parse_bit)
        .enumerate()
        .for_each(|(i, b)| enhancement[i] = b);

    input.next(); // skip empty line

    let mut image = [[false; N]; N];
    let margin = M;
    let mut size = None;
    for (i, line) in input.enumerate() {
        if size.is_none() {
            size = Some(line.len());
        }
        line.chars()
            .map(parse_bit)
            .enumerate()
            .for_each(|(j, b)| image[i + margin - 1][j + margin - 1] = b);
    }

    Image {
        enhancement,
        image,
        margin,
        size: size.unwrap(),
    }
}

fn parse_bit(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => unreachable!("invalid character {}", c),
    }
}

impl Image {
    fn range(&self) -> Range<usize> {
        self.margin - 1..self.size + self.margin - 1
    }

    pub fn count_lit(&self) -> usize {
        let mut n = 0;
        for y in self.range() {
            for x in self.range() {
                if self.image[y][x] {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn enhance_pixel(&self, x: usize, y: usize) -> bool {
        let x = x;
        let y = y;
        let i = &self.image;
        let pixels = [
            i[y - 1][x - 1],
            i[y - 1][x],
            i[y - 1][x + 1],
            i[y][x - 1],
            i[y][x],
            i[y][x + 1],
            i[y + 1][x - 1],
            i[y + 1][x],
            i[y + 1][x + 1],
        ];
        let e = pixels_to_number(&pixels);
        self.enhancement[e]
    }

    pub fn enhance_image(&self) -> Image {
        let base = self.enhance_pixel(1, 1);
        let image = [[base; N]; N];
        let margin = self.margin - 1;
        let size = self.size + 2;

        let mut new = Image {
            image,
            enhancement: self.enhancement,
            margin,
            size,
        };

        for y in new.range() {
            for x in new.range() {
                new.image[y][x] = self.enhance_pixel(x, y);
            }
        }
        new
    }
}

pub fn pixels_to_number(pixels: &[bool]) -> usize {
    debug_assert_eq!(pixels.len(), 9);
    let mut n = 0;
    for (i, &b) in pixels.iter().enumerate() {
        if b {
            n += 1 << (8 - i)
        }
    }
    n
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "size: {}, margin: {}", self.size, self.margin)?;
        for y in self.margin - 1..self.size + self.margin - 1 {
            for x in self.margin - 1..self.size + self.margin - 1 {
                write!(f, "{}", if self.image[y][x] { '#' } else { '.' })?;
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
    fn test_pixels_to_number() {
        let pixel = [false, false, false, true, false, false, false, true, false];
        let n = pixels_to_number(&pixel);
        assert_eq!(n, 34);
    }

    #[test]
    fn test_example() {
        let image = parse_input(include_str!("../input-example1"));
        println!("original image:\n{}", image);
        assert_eq!(image.count_lit(), 10);
        let e = image.enhance_pixel(3 + image.margin - 1, 3 + image.margin - 1);
        assert!(e);
        let image = image.enhance_image().enhance_image();
        println!("enhanced image:\n{}", image);
        assert_eq!(image.count_lit(), 35);
    }

    #[test]
    fn test_run_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 35);
        assert_eq!(part2_answer, 3351);
    }

    #[test]
    fn test_run_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 5249);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
