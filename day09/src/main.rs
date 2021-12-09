use std::collections::VecDeque;
use std::time::Instant;

const N: usize = 100;
const M: usize = 100;
type Heightmap = [[u8; M]; N];
type Floodmap = [[bool; M]; N];
type Node = (usize, usize);

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');
    let mut heightmap: Heightmap = [[0u8; M]; N];

    for (i, line) in input.enumerate() {
        for (j, d) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            heightmap[i][j] = d;
        }
    }

    let mut part1 = 0u32;
    let mut basins: Vec<u16> = Vec::new();

    for i in 0..N {
        for j in 0..M {
            let h = heightmap[i][j];
            if (j == 0 || heightmap[i][j - 1] > h)
                && (j == (M - 1) || heightmap[i][j + 1] > h)
                && (i == 0 || heightmap[i - 1][j] > h)
                && (i == (N - 1) || heightmap[i + 1][j] > h)
            {
                part1 += 1 + h as u32;
                basins.push(floodfill(heightmap, (i, j)));
            }
        }
    }

    basins.sort_unstable();
    basins.reverse();
    let part2 = basins[0] as u32 * basins[1] as u32 * basins[2] as u32;

    println!("part 1: {}", part1);
    assert_eq!(part1, 633);

    println!("part 2: {}", part2);
    assert_eq!(part2, 1050192);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}

// https://en.wikipedia.org/wiki/Flood_fill#Moving_the_recursion_into_a_data_structure
fn floodfill(heightmap: Heightmap, node: Node) -> u16 {
    let mut q: VecDeque<Node> = VecDeque::new();
    let mut floodmap: Floodmap = [[false; M]; N];
    let mut count = 0u16;
    q.push_back(node);
    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        if floodmap[i][j] || heightmap[i][j] == 9 {
            continue;
        }
        floodmap[i][j] = true;
        if j != 0 && !floodmap[i][j - 1] {
            q.push_back((i, j - 1));
        }
        if j != M - 1 && !floodmap[i][j + 1] {
            q.push_back((i, j + 1));
        }
        if i != 0 && !floodmap[i - 1][j] {
            q.push_back((i - 1, j));
        }
        if i != N - 1 && !floodmap[i + 1][j] {
            q.push_back((i + 1, j));
        }
        count += 1;
    }
    count
}
