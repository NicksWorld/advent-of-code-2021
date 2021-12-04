#![allow(unused)]

use std::prelude;

use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../input.txt")
        .trim()
        .split("\n")
        .collect();

    let drawings: Vec<u32> = input[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Vec<Vec<u32>>> = vec![];

    let mut tmp_board: Vec<Vec<u32>> = vec![];
    for line in &input[2..] {
        if *line == "" {
            // Start of new board
            boards.push(tmp_board);
            tmp_board = vec![];
        } else {
            let mut board_line = vec![];
            for item in line.split(" ") {
                if item != "" {
                    board_line.push(item.parse::<u32>().unwrap());
                }
            }
            tmp_board.push(board_line);
        }
    }
    boards.push(tmp_board);

    part_01(boards.clone(), drawings.clone());
    part_02(boards.clone(), drawings.clone());
}

fn check_board(board: &Vec<Vec<i32>>) -> bool {
    // Check rows
    for row in board {
        let mut is_complete = true;
        for item in row {
            if item < &0 {
                is_complete = false;
                break;
            }
        }

        if is_complete { return true; }
    }

    // Check columns
    for column in 0..board[0].len() {
        let mut is_complete = true;
        for row in board {
            if row[column] < 0 {
                is_complete = false;
                break;
            }
        }

        if is_complete { return true; }
    }

    false
}

fn score_board(marked_board: &Vec<Vec<i32>>, original_board: &Vec<Vec<u32>>, last_draw: u32) -> u32 {
    let mut unmarked_sum: u32 = 0;

    for row in 0..original_board.len() {
        for item in 0..original_board[0].len() {
            if marked_board[row][item] < 0 {
                unmarked_sum += original_board[row][item] as u32;
            }
        }
    }

    println!("Score {} x {}", unmarked_sum, last_draw);
    unmarked_sum * last_draw
}

fn mark_board(board: &Vec<Vec<u32>>, drawings: &[u32]) -> Vec<Vec<i32>>{
    let mut new_board: Vec<Vec<i32>> = vec![vec![-1i32; 5]; 5];

    for row in 0..board.len() {
        for item in 0..board[row].len() {
            if drawings.contains(&board[row][item]) {
                new_board[row][item] = board[row][item] as i32;
            }
        }
    }

    new_board
}

fn part_01(boards: Vec<Vec<Vec<u32>>>, drawings: Vec<u32>) {
    let mut complete = false;

    let mut num_drawings = 1;
    while(!complete) {
        for board in &boards {
            let marked = mark_board(board, &drawings[0..num_drawings]);

            if check_board(&marked) {
                println!("Board won on iteration {}", num_drawings);
                println!("Winning board's score: {}", score_board(&marked, board, drawings[num_drawings-1]));
                complete = true;
            }
        }

        num_drawings += 1;
    }
}

fn part_02(boards: Vec<Vec<Vec<u32>>>, drawings: Vec<u32>) {
    let mut board_list: Vec<Vec<Vec<u32>>> = boards;

    let mut num_drawings = 0;
    while(board_list.len() > 1 || !check_board(&mark_board(&board_list[0], &drawings[0..num_drawings+1]))) {
        num_drawings += 1;

        board_list = board_list
            .iter()
            .filter(|board| {
                let marked = mark_board(&board, &drawings[0..num_drawings]);

                score_board(&marked, &board_list[0], drawings[num_drawings-1]);

                !check_board(&marked)
            })
            .map(|x| x.to_owned())
            .collect::<Vec<Vec<Vec<u32>>>>();
    }

    println!("Found last board");
    let marked = mark_board(&board_list[0], &drawings[0..num_drawings+1]);
    println!("{:#?}", &board_list[0]);
    println!("Calculated score: {}", score_board(&marked, &board_list[0], drawings[num_drawings]));
}
