#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input: Vec<(&str, i32)> = include_str!("../input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let tmp = x.split(" ").collect::<Vec<&str>>();
            (tmp[0], tmp[1].parse::<i32>().unwrap())
        })
        .collect();

    part_01(input.clone());
    part_02(input.clone());
}

fn part_01(input: Vec<(&str, i32)>) {
    let mut depth = 0i32;
    let mut horiz = 0i32;

    for item in input {
        match item.0 {
            "forward" => horiz += item.1,
            "down" => depth += item.1,
            "up" => depth -= item.1,
            _ => println!("Unknown command: {}", item.0)
        }
    }

    println!("Horiz ({}) * Depth ({}) = {}", horiz, depth, horiz * depth);
}

fn part_02(input: Vec<(&str, i32)>) {
    let mut depth = 0i32;
    let mut horiz = 0i32;
    let mut aim = 0i32;

    for item in input {
        match item.0 {
            "forward" => {horiz += item.1; depth += aim * item.1;},
            "down" => aim += item.1,
            "up" => aim -= item.1,
            _ => println!("Unknown command: {}", item.0)
        }
    }

    println!("Horiz ({}) * Depth ({}) = {}", horiz, depth, horiz * depth);
}
