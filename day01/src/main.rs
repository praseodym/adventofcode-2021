#![feature(test)]
#![feature(array_windows)]

extern crate test;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let measurements = read_measurements();
    let increments_1 = first(&measurements);
    let increments_2 = second(&measurements);

    assert_eq!(increments_1, 1167);
    assert_eq!(increments_2, 1130);
    println!("First answer:  {}", increments_1);
    println!("Second answer: {}", increments_2);

    Result::Ok(())
}

fn read_measurements() -> Vec<i32> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line_str| line_str.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn first(measurements: &[i32]) -> i32 {
    measurements
        .array_windows()
        .map(|&[a, b]| if b > a { 1 } else { 0 })
        .sum::<i32>()
}

fn second(measurements: &[i32]) -> i32 {
    measurements
        .array_windows::<3>()
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .array_windows()
        .map(|&[a, b]| if b > a { 1 } else { 0 })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(main);
    }

    #[bench]
    fn bench_first(b: &mut Bencher) {
        let measurements = read_measurements();
        b.iter(|| first(&measurements));
    }

    #[bench]
    fn bench_second(b: &mut Bencher) {
        let measurements = read_measurements();
        b.iter(|| second(&measurements));
    }
}
