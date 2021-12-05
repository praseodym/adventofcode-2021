use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut grid = [[0u8; 1000]; 1000];

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // println!("line: {}", line);
        let coords: Vec<usize> = line
            .split(" -> ")
            .flat_map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()))
            .collect();

        let x1: usize;
        let y1: usize;
        let x2: usize;
        let y2: usize;

        if coords[0] <= coords[2] {
            x1 = coords[0];
            x2 = coords[2];
        } else {
            x1 = coords[2];
            x2 = coords[0];
        }
        if coords[1] <= coords[3] {
            y1 = coords[1];
            y2 = coords[3];
        } else {
            y1 = coords[3];
            y2 = coords[1];
        }

        // println!("coords: {},{} -> {},{}", x1, y1, x2, y2);

        // mark points in grid, considering only lines (x1 == x2 || y1 == y2) for now
        if x1 == x2 {
            for y in y1..y2+1 {
                // println!("Y-line mark point {},{}", x1, y);
                grid[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in x1..x2+1 {
                // println!("X-line mark point {},{}", x, y1);
                grid[y1][x] += 1;
            }
        } else {
            continue;
        }
    }

    // for y in 0..10 {
    //     for x in 0..10 {
    //         let cnt = grid[y][x];
    //         if cnt == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", cnt);
    //         }
    //     }
    //     println!(" -- {}", y);
    // }

    let mut answer1 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] >= 2 {
                answer1 += 1;
            }
        }
    }
    println!("answer 1: {}", answer1);
}
