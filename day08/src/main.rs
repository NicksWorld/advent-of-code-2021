#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split("\n")
        .map(|x| x.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    part_01(input.clone());
    part_02(input.clone());
}

fn part_01(input: Vec<Vec<&str>>) {
    let mut acc = 0;

    for line in &input {
        for item in line[1].split(" ") {
            let len = item.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                acc += 1;
            }
        }
    }

    println!("Part 1: {}", acc)
}

fn includes_all(src: &str, to_test: &str) -> bool {
    for character in src.chars() {
        if !to_test.contains(character) {
            return false;
        }
    }

    true
}

fn includes_except(src: &str, exception: &str, to_test: &str) -> bool {
    for character in src.chars() {
        if !to_test.contains(character) && !exception.contains(character) {
            return false;
        }
    }

    true
}


fn part_02(input: Vec<Vec<&str>>) {
    let mut one = "";
    let mut four = "";
    let mut seven = "";
    let mut eight = "";

    let mut all_outs = vec![];
    for line in &input {
        for item in line[0].split(" ") {
            match item.len() {
                2 => one = item,
                4 => four = item,
                3 => seven = item,
                7 => eight = item,
                _ => ()
            }
        }

        let mut out = 0;
        for digit in line[1].split(" ") {
            out *= 10;
            let mut output = -1;
            match digit.len() {
                2 => output = 1,
                4 => output = 4,
                3 => output = 7,
                7 => output = 8,
                _ => ()
            }
            // Covered numbers: x,1,2,3,4,5,x,7,8,x

            if output != -1 {
                //continue;
            }

            // 3, 2, 5
            if digit.len() == 5 {
                if includes_all(seven, digit) {
                    output = 3;
                } else if includes_except(four, one, digit) {
                    output = 5;
                } else {
                    output = 2;
                }
            }

            if digit.len() == 6 {
                if includes_all(seven, digit) && !includes_all(four, digit) {
                    output = 0;
                } else if includes_all(seven, digit) {
                    output = 9;
                } else {
                    output = 6;
                }
            }
            out += output;
        }
        println!("{}", out);
        all_outs.push(out);
    }

    println!("Part 2: {}", all_outs.iter().sum::<i32>());
}
