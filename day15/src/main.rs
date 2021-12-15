#![feature(test)]

extern crate test;

use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

const N: usize = 100; // x direction
const M: usize = 100; // y direction
type Risk = u16;
#[derive(Debug)]
struct Grid {
    risks: [[Risk; N]; M],
    width: usize,
    height: usize,
}
#[derive(Default, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
    risk: Risk,
}

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u16, u16) {
    let grid = Grid::parse_input(input);
    let part1_answer = grid.find_path();
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

impl Grid {
    fn parse_input(input: &str) -> Grid {
        let mut grid: Grid = Grid {
            risks: [[0u16; M]; N],
            width: 0,
            height: 0,
        };
        let input = input.trim_end().split('\n');
        for (y, line) in input.enumerate() {
            for (x, d) in line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as Risk)
                .enumerate()
            {
                grid.width = cmp::max(x, grid.width);
                grid.height = cmp::max(y, grid.height);
                grid.risks[y][x] = d;
            }
        }
        grid
    }

    // https://codereview.stackexchange.com/a/202879
    fn find_path(&self) -> u16 {
        let mut risks = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit: BinaryHeap<Position> = BinaryHeap::new();

        let adj = self.get_adjacent(Default::default());
        adj.iter().for_each(|p| to_visit.push(*p));

        while let Some(p) = to_visit.pop() {
            if !visited.insert(p) {
                continue;
            }

            let adj = self.get_adjacent(p);
            for neighbour in adj {
                let new_risk = p.risk + neighbour.risk;

                if neighbour.x == self.width && neighbour.y == self.height {
                    return new_risk;
                }

                let lower_risk = risks
                    .get(&neighbour)
                    .map_or(true, |&current| new_risk < current);

                if lower_risk {
                    risks.insert(neighbour, new_risk);
                    to_visit.push(Position {
                        x: neighbour.x,
                        y: neighbour.y,
                        risk: new_risk,
                    })
                }
            }
        }

        panic!("no valid route found");
    }

    // "you cannot move diagonally"
    fn get_adjacent(&self, pos: Position) -> Vec<Position> {
        let mut ret = Vec::new();
        let x = pos.x;
        let y = pos.y;
        if x != 0 {
            let x = x - 1;
            let y = y;
            let risk = self.risks[x][y];
            ret.push(Position { x, y, risk })
        }
        if x != self.width {
            let x = x + 1;
            let y = y;
            let risk = self.risks[x][y];
            ret.push(Position { x, y, risk })
        }
        if y != 0 {
            let x = x;
            let y = y - 1;
            let risk = self.risks[x][y];
            ret.push(Position { x, y, risk })
        }
        if y != self.height {
            let x = x;
            let y = y + 1;
            let risk = self.risks[x][y];
            ret.push(Position { x, y, risk })
        }
        ret
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test_parse() {
        let grid = Grid::parse_input(include_str!("../input-test"));
        assert_eq!(grid.width + 1, 10);
        assert_eq!(grid.height + 1, 10);
    }

    #[test]
    fn test_input_test() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-test"));
        assert_eq!(part1_answer, 40);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 696);
        // assert_eq!(part2_answer, 0);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
