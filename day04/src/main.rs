#![feature(drain_filter)]

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;

use crate::bingo::BingoCard;

mod bingo;

fn main() {
    let now = Instant::now();

    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf: String = String::new();
    reader.read_line(&mut buf).unwrap();
    let draws: Vec<u8> = buf
        .trim_end()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut cards = Vec::<BingoCard>::new();
    loop {
        reader.seek_relative(1).unwrap(); // skip newline
        let mut buf = [0u8; 5 * 15];
        // TODO: better EOF handling
        match reader.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => break,
        }
        let card = BingoCard::new(&buf);
        cards.push(card);
    }

    let mut first = None;
    let mut last = 0;
    for draw in draws {
        cards.drain_filter(|card| {
            if let Some(score) = card.mark(draw) {
                if first.is_none() {
                    first = Some(score)
                }
                last = score;
                true
            } else {
                false
            }
        });
    }

    let first = first.unwrap();
    assert_eq!(first, 38913);
    assert_eq!(last, 16836);
    println!("first to score: {}", first);
    println!("last to score: {}", last);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
