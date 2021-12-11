use std::collections::VecDeque;
use std::time::Instant;

const N: usize = 10;
const M: usize = 10;
type Octopuses = [[u8; M]; N];
type Octopus = (usize, usize);

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');
    let mut octopuses: Octopuses = [[0u8; M]; N];
    let mut flashes = 0u32;

    for (i, line) in input.enumerate() {
        assert_eq!(M, line.len());
        for (j, d) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            octopuses[i][j] = d;
        }
    }

    // debug(0, octopuses);

    for step in 1..=100 {
        let mut q: VecDeque<Octopus> = VecDeque::new();
        let mut r: VecDeque<Octopus> = VecDeque::new();
        for i in 0..N {
            for j in 0..M {
                increase_energy_level(&mut octopuses, (i, j), &mut q);
            }
        }
        while !q.is_empty() {
            let octopus = q.pop_front().unwrap();
            flash(&mut octopuses, octopus, &mut q);
            flashes += 1;
            r.push_back(octopus);
        }
        while !r.is_empty() {
            let (i, j) = r.pop_front().unwrap();
            octopuses[i][j] = 0;
        }
        // debug(step, octopuses);
    }

    println!("part 1: {} flashes", flashes);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}

fn increase_energy_level(octopuses: &mut Octopuses, octopus: Octopus, q: &mut VecDeque<Octopus>) {
    let (i, j) = octopus;
    octopuses[i][j] += 1;
    if octopuses[i][j] == 10 {
        q.push_back((i, j));
    }
}

fn flash(octopuses: &mut Octopuses, octopus: Octopus, q: &mut VecDeque<Octopus>) {
    let (i, j) = octopus;
    if i != 0 {
        if j != 0 {
            increase_energy_level(octopuses, (i - 1, j - 1), q);
        }
        increase_energy_level(octopuses, (i - 1, j), q);
        if j != M - 1 {
            increase_energy_level(octopuses, (i - 1, j + 1), q);
        }
    }
    if j != 0 {
        increase_energy_level(octopuses, (i, j - 1), q);
    }
    if j != M - 1 {
        increase_energy_level(octopuses, (i, j + 1), q);
    }
    if i != N - 1 {
        if j != 0 {
            increase_energy_level(octopuses, (i + 1, j - 1), q);
        }
        increase_energy_level(octopuses, (i + 1, j), q);
        if j != M - 1 {
            increase_energy_level(octopuses, (i + 1, j + 1), q);
        }
    }
}

fn debug(step: u32, octopuses: Octopuses) {
    println!("After step {}", step);
    for i in 0..N {
        for j in 0..M {
            print!("{}", octopuses[i][j])
        }
        println!();
    }
    println!();
}