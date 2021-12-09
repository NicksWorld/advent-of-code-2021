#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    part_01(input.clone());
    part_02(input.clone());
}

// I have no idea why I named it this
fn increasing_addition(num: usize) -> usize {
    let mut res = 0;
    for i in 1..=num {
        res += i;
    }
    return res;
}

fn part_01(input: Vec<i32>) {
    let range = (*input.iter().min().unwrap()..*input.iter().max().unwrap()).collect::<Vec<i32>>();
    println!("{:#?}", range);
    let mut best: Vec<usize> = vec![];

    for i in &range {
        let mut fuel_used: usize = 0;
        for crab in &input {
            fuel_used += increasing_addition((i - crab).abs() as usize);
        }

        best.push(fuel_used);
    }

    let mut abs_best = (range[0], best[0]);
    for i in 0..range.len() {
        if (abs_best.1 > best[i]) {
            abs_best = (range[i], best[i]);
        }
    }

    println!("{:#?}", abs_best);
}

fn part_02(input: Vec<i32>) {

}
