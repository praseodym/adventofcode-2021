#![feature(test)]

extern crate test;

use std::collections::VecDeque;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let start = parse_input(input);
    let part1_answer = part1(start);
    let part2_answer = part2(start);
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

fn part2(start: (usize, usize)) -> usize {
    let dice_freq = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    let mut wins: [usize; 2] = [0, 0];
    let mut q: VecDeque<DiracState> = VecDeque::new();
    q.push_back(DiracState {
        cur_player: 0,
        universes: 1,
        pos: [start.0 - 1, start.1 - 1],
        score: [0, 0],
    });

    while let Some(state) = q.pop_front() {
        #[allow(clippy::needless_range_loop)]
        for roll in 3..=9 {
            let universes = state.universes * dice_freq[roll];
            let mut pos = state.pos;
            let mut score = state.score;
            pos[state.cur_player] = (state.pos[state.cur_player] + roll) % 10;
            score[state.cur_player] += pos[state.cur_player] + 1;
            if score[state.cur_player] >= 21 {
                wins[state.cur_player] += universes;
                continue;
            }
            q.push_back(DiracState {
                cur_player: (state.cur_player + 1) % 2,
                universes,
                pos,
                score,
            })
        }
    }

    std::cmp::max(wins[0], wins[1])
}

#[derive(Debug, Default)]
struct DiracState {
    cur_player: usize,
    universes: usize,
    pos: [usize; 2],
    score: [usize; 2],
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
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 739785);
        assert_eq!(part2_answer, 444356092776315);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1196172);
        assert_eq!(part2_answer, 106768284484217);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
