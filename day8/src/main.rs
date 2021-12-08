use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');

    let mut part1 = 0u32;
    let mut part2 = 0u32;

    for line in input {
        let mut s = line.split('|');

        let signals_raw = s.next().unwrap().trim();

        let mut signals_sorted = signals_raw.split(' ').collect::<Vec<&str>>();
        signals_sorted.sort_by_key(|a| a.len());

        let mut signals_occurences: HashMap<char, u8> = HashMap::new();
        signals_raw.chars().filter(|&c| c != ' ').for_each(|c| {
            signals_occurences
                .entry(c)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        });

        let mut mapping: HashMap<char, char> = HashMap::new();
        for (c, n) in signals_occurences {
            let m = match n {
                4 => 'e',
                6 => 'b',
                7 => {
                    // d and g occurs 7 times - d is in 4 (len 4), g is not
                    match signals_sorted[2].chars().find(|&p| p == c) {
                        Some(_) => 'd',
                        None => 'g',
                    }
                }
                8 => {
                    // a and c occur 8 times - a is not in 1 (len 2), c is
                    match signals_sorted[0].chars().find(|&p| p == c) {
                        Some(_) => 'c',
                        None => 'a',
                    }
                }
                9 => 'f',
                _ => {
                    panic!("{} occurred {} times", c, n);
                }
            };
            mapping.insert(c, m);
        }

        let outputs = s.next().unwrap().trim().split(' ');

        let mut digits = "".to_string();
        for output in outputs {
            let mut mapped = output
                .chars()
                .map(|c| mapping.get(&c).unwrap())
                .collect::<Vec<&char>>();
            mapped.sort();
            let new = mapped.into_iter().collect::<String>();

            digits += match new.as_str() {
                "abcefg" => "0",
                "cf" => {
                    part1 += 1;
                    "1"
                }
                "acdeg" => "2",
                "acdfg" => "3",
                "bcdf" => {
                    part1 += 1;
                    "4"
                }
                "abdfg" => "5",
                "abdefg" => "6",
                "acf" => {
                    part1 += 1;
                    "7"
                }
                "abcdefg" => {
                    part1 += 1;
                    "8"
                }
                "abcdfg" => "9",
                _ => {
                    panic!("unknown segment combination: {}", new);
                }
            }
        }
        part2 += digits.parse::<u32>().unwrap();
    }

    println!("part one: {}", part1);
    println!("part two: {}", part2);

    assert_eq!(part1, 416);
    assert_eq!(part2, 1043697);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros());
}
