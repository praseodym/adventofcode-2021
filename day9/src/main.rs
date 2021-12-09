use std::time::Instant;

fn main() {
    let now = Instant::now();

    let heightmap = include_str!("../input")
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let line_len = 100;

    let mut part1 = 0u32;

    let end = heightmap.len() - 1;
    for (i, &h) in heightmap.iter().enumerate() {
        if (i % line_len == 0 || heightmap[i - 1] > h)
            && (i % line_len == line_len || heightmap[i + 1] > h)
            && (i < line_len || heightmap[i - line_len] > h)
            && (i + line_len > end || heightmap[i + line_len] > h)
        {
            part1 += 1 + h as u32;
        }
    }

    println!("part 1: {}", part1);
    assert_eq!(part1, 633);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}
