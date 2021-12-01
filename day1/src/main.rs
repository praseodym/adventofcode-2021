use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut previous_measurement = None;
    let mut increments_1 = 0;
    let mut increments_2 = 0;
    let mut sliding_window = VecDeque::new();

    for (_, line) in reader.lines().enumerate() {
        let new_measurement = line?.parse::<i32>()?;

        // First strategy
        if let Some(d) = previous_measurement {
            if new_measurement > d {
                increments_1 += 1;
            }
        }
        previous_measurement = Some(new_measurement);

        // Second strategy with sliding window
        if sliding_window.len() == 3 {
            let first_sum: i32 = sliding_window.iter().sum();
            sliding_window.pop_front();
            sliding_window.push_back(new_measurement);
            let second_sum: i32 = sliding_window.iter().sum();
            if second_sum > first_sum {
                increments_2 += 1;
            }
        } else {
            sliding_window.push_back(new_measurement);
        };
    }

    assert_eq!(increments_1, 1167);
    assert_eq!(increments_2, 1130);
    println!("First answer:  {}", increments_1);
    println!("Second answer: {}", increments_2);

    Result::Ok(())
}
