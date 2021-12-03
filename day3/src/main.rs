mod bitwise_trie;

use crate::bitwise_trie::Trie;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut popcounts: [i32; 12] = [0; 12];
    let mut lines = 0;

    let mut trie = Trie::new();

    for (_, line) in reader.lines().enumerate() {
        lines += 1;
        let line = line.unwrap();
        let val = i32::from_str_radix(&line, 2).unwrap();
        trie.insert(val);
        for (i, bit) in line.chars().enumerate() {
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

    let o2gen = trie.o2_generator_rating();
    let co2scrub = trie.co2_scrubber_rating();
    println!("O2 generator rating: {}", o2gen);
    println!("CO2 scrubber rating: {}", co2scrub);
    println!("life support rating: {}", o2gen * co2scrub);

    assert_eq!(gamma, 2987);
    assert_eq!(epsilon, 1108);
    assert_eq!(o2gen, 2815);
    assert_eq!(co2scrub, 1059);
}
