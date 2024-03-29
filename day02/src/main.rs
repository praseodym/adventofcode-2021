fn main() {
    let input = include_str!("../input");

    let mut depth = 0;
    let mut horizontal = 0;

    let mut aim = 0;
    let mut depth_new = 0;

    for (_, line) in input.lines().enumerate() {
        let cmd = line;
        let cmd: Vec<&str> = cmd.split_whitespace().collect();
        let units = cmd[1].parse::<i32>().unwrap();
        let direction = cmd[0];
        match direction {
            "down" => {
                depth += units;
                aim += units;
            }
            "up" => {
                depth -= units;
                aim -= units;
            }
            "forward" => {
                horizontal += units;
                depth_new += aim * units;
            }
            _ => {
                panic!("invalid direction: {}", direction)
            }
        }
    }

    let answer1 = horizontal * depth;
    let answer2 = horizontal * depth_new;

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}
