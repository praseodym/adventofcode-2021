use std::cmp;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let positions: Vec<i32> = include_str!("../input")
        .trim_end()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut cheapest_fuel_part1 = i32::MAX;
    let mut cheapest_fuel_part2 = i32::MAX;

    for target_position in 0..1000 {
        let mut fuel_part1 = 0i32;
        let mut fuel_part2 = 0i32;
        for cur_position in &positions {
            let diff = (target_position - cur_position).abs();
            fuel_part1 += diff;
            for d in 1..=diff {
                fuel_part2 += d;
            }
        }
        cheapest_fuel_part1 = cmp::min(cheapest_fuel_part1, fuel_part1);
        cheapest_fuel_part2 = cmp::min(cheapest_fuel_part2, fuel_part2);
    }

    println!("cheapest part 1: {:?} fuel", cheapest_fuel_part1);
    println!("cheapest part 2: {:?} fuel", cheapest_fuel_part2);
    assert_eq!(344535, cheapest_fuel_part1);
    assert_eq!(95581659, cheapest_fuel_part2);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
