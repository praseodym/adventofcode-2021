#![feature(test)]

extern crate test;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let start = parse_input(input);
    let part1_answer = part1(start);
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> (usize, usize) {
    let mut input = input.trim_end().split('\n');
    let start_player1 = input
        .next()
        .unwrap()
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let start_player2 = input
        .next()
        .unwrap()
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    (start_player1, start_player2)
}

fn part1(start: (usize, usize)) -> usize {
    let mut die: DeterministicDie = Default::default();
    let mut pos: [usize; 2] = [start.0, start.1];
    let mut score: [usize; 2] = [0, 0];

    loop {
        for player in 0..2 {
            let roll = die.roll3();
            pos[player] = (pos[player] + roll - 1) % 10 + 1;
            score[player] += pos[player];
            if score[player] >= 1000 {
                return score[(player + 1) % 2] * die.count;
            }
        }
    }
}

#[derive(Debug, Default)]
struct DeterministicDie {
    state: usize,
    count: usize,
}

impl DeterministicDie {
    fn roll(&mut self) -> usize {
        self.count += 1;
        self.state += 1;
        if self.state == 101 {
            self.state = 1;
        }
        self.state
    }

    fn roll3(&mut self) -> usize {
        self.roll() + self.roll() + self.roll()
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_parse_example() {
        let (start_player1, start_player2) = parse_input(include_str!("../input-example"));
        assert_eq!(start_player1, 4);
        assert_eq!(start_player2, 8);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 739785);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1196172);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
