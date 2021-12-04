#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

const bit_count: usize = 12;
const bit_mask: u32 = 0b111111111111;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt").trim().split("\n").collect();

    part_01(input.clone());
}

fn part_01(input: Vec<&str>) {
    let result = input
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| if y == '0' { -1 } else { 1 })
                .collect::<Vec<i32>>()
        })
        .fold(vec![0i32; bit_count], |mut a, item| {
            (0..bit_count).for_each(|i| a[i] += item[i]);
            a
        })
        .iter()
        .fold((0, 0), |a, x| {
            (
                a.0 | ((if *x > 0 { true } else { false }) as u32) << (bit_count - 1 - a.1),
                a.1 + 1,
            )
        });

    let gamma = result.0;
    let epsilon = gamma ^ bit_mask;

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!(
        "Power consumption: {} x {} = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    part_02(input);
}

fn part_02(input: Vec<&str>) {
    let mut oxy_gen = 0;
    let mut carb_scrub = 0;

    for variant in 0..2 {
        let mut filtered_input = input
            .iter()
            .map(|x| {
                x.chars()
                    .fold((0, 0u32), |(i, acc), item| {
                        (
                            i + 1,
                            if item == '1' {
                                acc | (1 << (bit_count - 1 - i))
                            } else {
                                acc
                            },
                        )
                    })
                    .1
            })
            .collect::<Vec<u32>>();

        let mut bit = bit_count as i32 - 1;
        while (filtered_input.len() > 1) {
            let desired_bit_tmp = filtered_input.iter().fold(0, |acc, item| {
                if (item >> bit) & 1 == 0 {
                    acc - 1
                } else {
                    acc + 1
                }
            });

            let desired_bit: u8 = if variant == 0 {
                (desired_bit_tmp >= 0) as u8
            } else {
                (desired_bit_tmp < 0) as u8
            };

            filtered_input = filtered_input
                .iter()
                .filter(|x| (**x >> bit) & 1 == desired_bit as u32)
                .map(|x| *x)
                .collect::<Vec<u32>>();

            bit -= 1;
        }

        if variant == 0 {
            println!("Oxygen Generator Rating: {}", filtered_input[0]);
            oxy_gen = filtered_input[0];
        } else {
            println!("CO2 Scrubber Rating: {}", filtered_input[0]);
            carb_scrub = filtered_input[0];
        }
    }

    println!("Life support rating: {}", oxy_gen * carb_scrub);
}
