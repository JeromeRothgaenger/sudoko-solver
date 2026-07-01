#![allow(dead_code)]

use std::{iter, time::Instant};

use crate::helper::generate_mask;

mod display;
mod helper;
mod solver;
mod checker;
mod init;

const START_PUZZLE: usize = 1; // not 0
const PUZZEL_AMOUNT: usize = 49152;


fn main() {
 
    let start = Instant::now();

    let mut num_false: u32 = 0;
    let mut max_used_iterations = 0;

    
    for n in START_PUZZLE..START_PUZZLE + PUZZEL_AMOUNT {
        let mut state = init::read_sudoku(n);
        let iterations = solver::solve(&mut state);
        if iterations < 0 {
            println!("{} not solved, code {}", n, iterations);
            num_false += 1
        }
        if iterations > max_used_iterations {
            max_used_iterations = iterations;
        } 
    }
    let duration = start.elapsed().as_micros();
    println!("Time elapsed: {:?}s, avg time: {}mus", duration/1000000, duration/ PUZZEL_AMOUNT as u128);
    println!("total:{}; false: {}; false %: {}", PUZZEL_AMOUNT, num_false, (num_false as f32/PUZZEL_AMOUNT as f32)*100 as f32);
    println!("Maximum iterations: {}", max_used_iterations);
}