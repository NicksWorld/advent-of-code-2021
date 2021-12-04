use std::prelude;

use std::collections::HashMap;

fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let mut input = include_str!("../input.txt").trim().split("\n");

    let mut previous_num = input.next().unwrap().parse::<u32>().unwrap();
    let mut cum_increase = 0;

    for line in input {
        let new_num = line.parse::<u32>().unwrap();
        if previous_num < new_num {
            cum_increase += 1;
        }

        previous_num = new_num;
    }

    println!("Part01: {}", cum_increase);
}

fn part_02() {
    let input = include_str!("../input.txt")
        .trim()
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut previous_num = input[0..3]
        .iter()
        .fold(0, |sum, i| sum + i);
    let mut cum_increase = 0;

    for line_num in 0..input.len()-2 {
        let new_num = input[line_num..line_num+3]
            .iter()
            .fold(0, |sum, i| sum + i);
        if (previous_num < new_num) {
            cum_increase += 1;
        }

        previous_num = new_num;
    }

    println!("Part02: {}", cum_increase);

}
