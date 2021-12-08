use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');
    let mut digits = [0u32; 10];

    for line in input {
        let mut s = line.split('|');
        let signals = s.next().unwrap();
        let outputs = s.next().unwrap().trim().split(' ');

        for output in outputs {
            println!("output: {:?}", output);
            let digit = match output.len() {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                _ => None,
            };
            if let Some(d) = digit {
                digits[d] += 1;
            }
        }
    }

    println!(
        "part one: {}",
        digits[1] + digits[4] + digits[7] + digits[8]
    );

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
