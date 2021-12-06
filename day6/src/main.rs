use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut buf: String = String::new();
    reader.read_line(&mut buf).unwrap();
    let mut fishies: Vec<u8> = buf
        .trim_end()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    for _day in 0..80 {
        let mut spawned = Vec::<u8>::new();
        for fish in &mut fishies {
            if *fish == 0 {
                spawned.push(8);
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }
        fishies.append(&mut spawned);
        // println!("fishies on day {}: {:?}", _day, fishies);
    }
    println!("fishies: {}", fishies.len());
}
