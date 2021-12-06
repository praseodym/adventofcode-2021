use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    type Fishies = [u64; 9];
    let mut fishies: Fishies = Default::default();

    let mut buf: String = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.trim_end()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|f| fishies[f] += 1);

    for day in 1..=256 {
        let mut new_fishies: Fishies = Default::default();
        // spawn new fishies..
        new_fishies[8] = fishies[0];
        // .. and reset their timers
        new_fishies[6] = fishies[0];
        // decrement timer for all other fishies
        for i in 1..=8 {
            new_fishies[i - 1] += fishies[i];
        }
        fishies = new_fishies;
        if day == 80 || day == 256 {
            let sum: u64 = fishies.iter().sum();
            if day == 80 {
                assert_eq!(sum, 352195);
            } else if day == 256 {
                assert_eq!(sum, 1600306001288);
            }
            println!("fishies on day {}: {}", day, sum);
        }
    }

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
