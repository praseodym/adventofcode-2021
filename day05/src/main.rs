use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut grid1 = [[0u8; 1000]; 1000];
    let mut grid2 = [[0u8; 1000]; 1000];

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // println!("line: {}", line);
        let coords: Vec<usize> = line
            .split(" -> ")
            .flat_map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()))
            .collect();

        let x1 = coords[0];
        let y1 = coords[1];
        let x2 = coords[2];
        let y2 = coords[3];

        // println!("coords: {},{} -> {},{}", x1, y1, x2, y2);

        // mark points in grid, considering only lines (x1 == x2 || y1 == y2) for grid1
        if x1 == x2 && y1 <= y2 {
            for y in y1..y2 + 1 {
                grid1[y][x1] += 1;
                grid2[y][x1] += 1;
            }
        } else if x1 == x2 && y1 > y2 {
            for y in y2..y1 + 1 {
                grid1[y][x1] += 1;
                grid2[y][x1] += 1;
            }
        } else if y1 == y2 && x1 <= x2 {
            for x in x1..x2 + 1 {
                grid1[y1][x] += 1;
                grid2[y1][x] += 1;
            }
        } else if y1 == y2 && x1 > x2 {
            for x in x2..x1 + 1 {
                grid1[y1][x] += 1;
                grid2[y1][x] += 1;
            }
        } else if x1 <= x2 && y1 <= y2 {
            let mut x = x1;
            for y in y1..y2 + 1 {
                grid2[y][x] += 1;
                x += 1;
            }
        } else if x1 > x2 && y1 <= y2 {
            let mut x = x1 + 1;
            for y in y1..y2 + 1 {
                x -= 1;
                grid2[y][x] += 1;
            }
        } else if x1 <= x2 && y1 > y2 {
            let mut x = x1 - 1;
            let mut y = y1 + 1;
            loop {
                x += 1;
                y -= 1;
                grid2[y][x] += 1;
                if x == x2 {
                    break;
                }
            }
        } else if x1 > x2 && y1 > y2 {
            let mut x = x1 + 1;
            let mut y = y1 + 1;
            loop {
                x -= 1;
                y -= 1;
                grid2[y][x] += 1;
                if x == x2 {
                    break;
                }
            }
        } else {
            println!("wut? {},{} -> {},{}", x1, x2, y1, y2);
        }
    }

    // for y in 0..10 {
    //     for x in 0..10 {
    //         let cnt = grid2[y][x];
    //         if cnt == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", cnt);
    //         }
    //     }
    //     println!(" -- {}", y);
    // }

    let mut answer1 = 0;
    let mut answer2 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid1[x][y] >= 2 {
                answer1 += 1;
            }
            if grid2[x][y] >= 2 {
                answer2 += 1;
            }
        }
    }
    assert_eq!(answer1, 5698);
    assert_eq!(answer2, 15463);
    println!("answer 1: {}", answer1);
    println!("answer 2: {}", answer2);
}
