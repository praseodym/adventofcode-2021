use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut popcounts: [i32; 12] = [0; 12];
    let mut lines = 0;

    for (_, line) in reader.lines().enumerate() {
        lines += 1;
        for (i, bit) in line.unwrap().chars().enumerate() {
            match bit {
                '0' => {}
                '1' => {
                    popcounts[i] += 1;
                }
                _ => {
                    panic!("invalid bit: {}", bit)
                }
            }
        }
    }

    let threshold = lines / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, &popcount) in popcounts.iter().enumerate() {
        let bitval = i32::pow(2, (11 - i) as u32);
        if popcount > threshold {
            gamma += bitval;
        } else {
            epsilon += bitval;
        }
    }

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("power consumption: {}", gamma * epsilon);
}
