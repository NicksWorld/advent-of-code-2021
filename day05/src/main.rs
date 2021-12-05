#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let tmp = x.split(" -> ")
                .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                .collect::<Vec<Vec<u32>>>();

            ((tmp[0][0], tmp[0][1]), (tmp[1][0], tmp[1][1]))
        })
        .collect::<Vec<((u32, u32), (u32, u32))>>();

    part_01(input.clone());
}

fn part_01(lines: Vec<((u32, u32), (u32, u32))>) {
    let mut grid: Vec<Vec<u32>> = vec![vec![0u32; 1000]; 1000];

    for line in &lines {
        if (line.0.0 == line.1.0) {
            // Vertical
            let mut y1 = line.0.1;
            let mut y2 = line.1.1;
            if(y1 > y2) {
                std::mem::swap(&mut y1, &mut y2);
            }

            for coord in y1..=y2 {
                grid[coord as usize][line.0.0 as usize] += 1;
            }
        } else if (line.0.1 == line.1.1) {
            // Horizontal
            let mut x1 = line.0.0;
            let mut x2 = line.1.0;
            if(x1 > x2) {
                std::mem::swap(&mut x1, &mut x2);
            }

            for coord in x1..=x2 {
                grid[line.0.1 as usize][coord as usize] += 1;
            }
        } else {
            // PART TWO ONLY!!!
            // 45deg diagonal
            let mut xs: Vec<u32> = (line.0.0..=line.1.0)
                .collect();
            if(xs.is_empty()) {
                xs = (line.1.0..=line.0.0)
                    .collect();
                xs = xs.iter().rev().map(|x| x.to_owned()).collect::<Vec<u32>>();
            }

            let mut ys: Vec<u32> = (line.0.1..=line.1.1)
                .collect();
            if(ys.is_empty()) {
                ys = (line.1.1..=line.0.1)
                    .collect();
                ys = ys.iter().rev().map(|x| x.to_owned()).collect::<Vec<u32>>();
            }

            for (x, y) in (xs.iter()).zip(ys.iter()) {
                println!("{},{}", x, y);
                grid[*y as usize][*x as usize] += 1;
            }
        }
    }

    // Count overlaps
    let mut total_overlaps = 0;
    for row in &grid {
        for item in row {
            if item >= &2 {
                total_overlaps += 1;
            }
        }
    }

    //println!("{:?}", grid);
    println!("Total overlaps (part1): {}", total_overlaps);
}
