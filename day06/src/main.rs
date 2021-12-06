#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    part_01(input.clone());
    part_02(input.clone());
}

fn part_01(input: Vec<u32>) {
    let mut ages = input;

    for iteration in 0..80 {
        for i in 0..ages.len() {
            if ages[i] == 0 {
                ages[i] = 6;
                ages.push(8);
            } else {
                ages[i] -= 1;
            }
        }
    }

    println!("Part 1: {}", ages.len());
}

fn part_02(input: Vec<u32>) {
    let mut age_each = vec![0usize; 9];

    for age in input {
        age_each[age as usize] += 1;
    }

    for iteration in 0..256 {
        let mut new_age_each = vec![0usize; 9];
        let age_zeroes = age_each[0];
        age_each[0] = 0;

        for item in 1..=8 {
            new_age_each[item-1] = age_each[item];
        }

        new_age_each[6] += age_zeroes;
        new_age_each[8] += age_zeroes;

        age_each = new_age_each;
    }
    println!("Part 2: {}", age_each.iter().sum::<usize>());
}
