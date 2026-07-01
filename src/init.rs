use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::helper;

pub(crate) fn read_sudoku(num: usize) -> [u16; 81] {
    let file = File::open("all_17_clue_sudokus.txt").expect("Could not open file");
    let line = BufReader::new(file)
        .lines()
        .nth(num)
        .expect("No such line")
        .expect("Could not read line");

    let mut state = [0b_011_1111_1110; 81];
    for (i, c) in line.chars().enumerate() {
        if c != '0' {
            helper::set_field(&mut state, i, 1 << (c as u8 - b'0'));
        }
    }

    state
}