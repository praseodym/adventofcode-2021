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
        let one = signals_raw.split(' ').find(|s| s.len() == 2).unwrap();
        let four = signals_raw.split(' ').find(|s| s.len() == 4).unwrap();

        let mut signals_occurences: HashMap<char, u8> = HashMap::with_capacity(7);
        signals_raw.chars().filter(|&c| c != ' ').for_each(|c| {
            signals_occurences
                .entry(c)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        });

        let mut mapping: HashMap<char, char> = HashMap::with_capacity(7);
        for (c, n) in signals_occurences {
            let m = match n {
                4 => 'e',
                6 => 'b',
                7 => {
                    // d and g occur 7 times - d is in 4, g is not
                    if four.contains(c) {
                        'd'
                    } else {
                        'g'
                    }
                }
                8 => {
                    // a and c occur 8 times - c is in 1, a is not
                    if one.contains(c) {
                        'c'
                    } else {
                        'a'
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

            let d = match new.as_str() {
                "abcefg" => '0',
                "cf" => '1',
                "acdeg" => '2',
                "acdfg" => '3',
                "bcdf" => '4',
                "abdfg" => '5',
                "abdefg" => '6',
                "acf" => '7',
                "abcdefg" => '8',
                "abcdfg" => '9',
                _ => {
                    panic!("unknown segment combination: {}", new);
                }
            };
            if d == '1' || d == '4' || d == '7' || d == '8' {
                part1 += 1;
            }
            digits.push(d);
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
