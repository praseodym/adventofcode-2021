#![feature(test)]

extern crate test;

type TargetArea = ((i32, i32), (i32, i32));

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (i32, usize) {
    let target = parse_input(input);
    let part1_answer = max_y(target).unwrap();
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> TargetArea {
    let mut s = input
        .trim_end()
        .strip_prefix("target area: ")
        .unwrap()
        .split(", ");
    let mut x = s.next().unwrap().strip_prefix("x=").unwrap().split("..");
    let mut y = s.next().unwrap().strip_prefix("y=").unwrap().split("..");
    let x1: i32 = x.next().unwrap().parse().unwrap();
    let x2: i32 = x.next().unwrap().parse().unwrap();
    let y1: i32 = y.next().unwrap().parse().unwrap();
    let y2: i32 = y.next().unwrap().parse().unwrap();
    ((x1, x2), (y1, y2))
}

fn simulate(velocity: (i32, i32), target: TargetArea) -> Option<i32> {
    let mut dx = velocity.0;
    let mut dy = velocity.1;
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    // println!("\nstarting simulation");
    for _step in 0..250 {
        // println!("step {}: x,y {},{} dx,dy {},{}", step, x, y, dx, dy);

        // The probe's x position increases by its x velocity.
        x += dx;
        // The probe's y position increases by its y velocity.
        y += dy;
        // Due to drag, the probe's x velocity changes by 1 toward the value 0;
        // that is, it decreases by 1 if it is greater than 0, increases by 1 if
        // it is less than 0, or does not change if it is already 0.
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
        // Due to gravity, the probe's y velocity decreases by 1.
        dy -= 1;

        // Keep track of highest y position
        max_y = std::cmp::max(y, max_y);

        // Check whether probe is in target area
        if x >= target.0 .0 && x <= target.0 .1 && y >= target.1 .0 && y <= target.1 .1 {
            // println!("probe is in target area!");
            // println!("step {}: x,y {},{} dx,dy {},{}", step, x, y, dx, dy);
            return Some(max_y);
        }
    }
    None
}

fn max_y(target: TargetArea) -> Option<i32> {
    let bound_x = std::cmp::max(target.0.0.abs(), target.0.1.abs());
    let bound_y = std::cmp::max(target.1.0.abs(), target.1.1.abs());
    let mut max_y = None;
    for dx in -bound_x..bound_x {
        for dy in -bound_y..bound_y {
            if let Some(new_max_y) = simulate((dx, dy), target) {
                max_y = match max_y {
                    Some(old_max_y) => Some(std::cmp::max(old_max_y, new_max_y)),
                    None => Some(new_max_y),
                }
            }
        }
    }
    max_y
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_parse_test() {
        let target = parse_input(include_str!("../input-test"));
        assert_eq!(target, ((20, 30), (-10, -5)));
    }

    #[test]
    fn test_parse_own() {
        let target = parse_input(include_str!("../input"));
        assert_eq!(target, ((241, 275), (-75, -49)));
    }

    #[test]
    fn test_simulate_test() {
        let target = parse_input(include_str!("../input-test"));

        let max_y = simulate((6, 3), target);
        assert!(max_y.is_some());

        let max_y = simulate((9, 0), target);
        assert!(max_y.is_some());

        let max_y = simulate((17, -4), target);
        assert!(max_y.is_none());

        let max_y = simulate((6, 9), target);
        assert!(max_y.is_some());
        assert_eq!(max_y.unwrap(), 45);
    }

    #[test]
    fn test_input_test() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-test"));
        assert_eq!(part1_answer, 45);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 2775);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
