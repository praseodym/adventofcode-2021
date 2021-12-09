use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');

    const N: usize = 100;
    const M: usize = 100;
    let mut heightmap = [[0u8; M]; N];

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

    for i in 0..N {
        for j in 0..M {
            let h = heightmap[i][j];
            if (i == 0 || heightmap[i - 1][j] > h)
                && (i == (N - 1) || heightmap[i + 1][j] > h)
                && (j == 0 || heightmap[i][j - 1] > h)
                && (j == (M - 1) || heightmap[i][j + 1] > h)
            {
                part1 += 1 + h as u32;
            }
        }
    }

    println!("part 1: {}", part1);
    assert_eq!(part1, 633);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}
