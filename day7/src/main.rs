use std::time::Instant;

fn main() {
    let now = Instant::now();

    let positions: Vec<i32> = include_str!("../input")
        .trim_end()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut cheapest_fuel: Option<i32> = None;

    for target_position in 0..1000 {
        let mut fuel = 0i32;
        for cur_position in &positions {
            fuel += (target_position - cur_position).abs();
        }
        match cheapest_fuel {
            Some(f) => {
                if fuel < f {
                    cheapest_fuel = Some(fuel);
                }
            }
            None => {
                cheapest_fuel = Some(fuel);
            }
        }
    }

    println!("cheapest: {:?} fuel", cheapest_fuel.unwrap());

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
