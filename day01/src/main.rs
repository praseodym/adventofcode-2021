fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut previous_measurement = None;
    let mut increments_1 = 0;
    let mut increments_2 = 0;
    let mut sliding_window = std::collections::VecDeque::new();

    for (_, line) in input.lines().enumerate() {
        let new_measurement = line.parse::<i32>().unwrap();

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
    (increments_1, increments_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1167);
        assert_eq!(part2_answer, 1130);
    }
}
