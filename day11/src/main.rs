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
    let mut first_step_all_flash = 0u32;

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

    let mut step = 0u32;
    while step < 100 || first_step_all_flash == 0 {
        step += 1;
        let mut flashes_round = 0u32;
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
            if step <= 100 {
                flashes += 1;
            }
            flashes_round += 1;
            r.push_back(octopus);
        }
        while !r.is_empty() {
            let (i, j) = r.pop_front().unwrap();
            octopuses[i][j] = 0;
        }
        if flashes_round == 100 && first_step_all_flash == 0 {
            first_step_all_flash = step;
        }
        // debug(step, octopuses);
    }

    println!("part 1: {} flashes", flashes);
    println!(
        "part 2: {} is the first step during which all octopuses flash",
        first_step_all_flash
    );

    assert_eq!(flashes, 1632);
    assert_eq!(first_step_all_flash, 303);

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
    for dx in -1..=1 {
        for dy in -1..=1 {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if x >= 0
                && y >= 0
                && (x as usize) < N
                && (y as usize) < M
                && !(x as usize == i && y as usize == j)
            {
                increase_energy_level(octopuses, (x as usize, y as usize), q);
            }
        }
    }
}

#[allow(dead_code)]
fn debug(step: u32, octopuses: Octopuses) {
    println!("After step {}", step);
    for row in octopuses {
        for octopus in row {
            print!("{}", octopus)
        }
        println!();
    }
    println!();
}
