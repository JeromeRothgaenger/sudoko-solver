use std::cell;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

use std::collections::HashMap;



const ROW_LIST: [[usize; 9]; 9] = [[0, 1, 2, 3, 4, 5, 6, 7, 8], [9, 10, 11, 12, 13, 14, 15, 16, 17], [18, 19, 20, 21, 22, 23, 24, 25, 26], [27, 28, 29, 30, 31, 32, 33, 34, 35], [36, 37, 38, 39, 40, 41, 42, 43, 44], [45, 46, 47, 48, 49, 50, 51, 52, 53], [54, 55, 56, 57, 58, 59, 60, 61, 62], [63, 64, 65, 66, 67, 68, 69, 70, 71], [72, 73, 74, 75, 76, 77, 78, 79, 80]];
const COL_LIST: [[usize; 9]; 9] = [[0, 9, 18, 27, 36, 45, 54, 63, 72], [1, 10, 19, 28, 37, 46, 55, 64, 73], [2, 11, 20, 29, 38, 47, 56, 65, 74], [3, 12, 21, 30, 39, 48, 57, 66, 75], [4, 13, 22, 31, 40, 49, 58, 67, 76], [5, 14, 23, 32, 41, 50, 59, 68, 77], [6, 15, 24, 33, 42, 51, 60, 69, 78],  [7, 16, 25, 34, 43, 52, 61, 70, 79], [8, 17, 26, 35, 44, 53, 62, 71, 80]];
const BLK_LIST: [[usize; 9]; 9] = [[0, 1, 2, 9, 10, 11, 18, 19, 20], [3, 4, 5, 12, 13, 14, 21, 22, 23], [6, 7, 8, 15, 16, 17, 24, 25, 26], [27, 28, 29, 36, 37, 38, 45, 46, 47], [30, 31, 32, 39, 40, 41, 48, 49, 50], [33, 34, 35, 42, 43, 44, 51, 52, 53], [54, 55, 56, 63, 64, 65, 72, 73, 74], [57, 58, 59, 66, 67, 68, 75, 76, 77], [60, 61, 62, 69, 70, 71, 78, 79, 80]];
const BLK_LIST_VER: [[usize; 9]; 9] = [[0, 9, 18, 1, 10, 19, 2, 11, 20], [3, 12, 21, 4, 13, 22, 5, 14, 23], [6, 15, 24, 7, 16, 25, 8, 17, 26], [27, 36, 45, 28, 37, 46, 29, 38, 47], [30, 39, 48, 31, 40, 49, 32, 41, 50], [33, 42, 51, 34, 43, 52, 35, 44, 53], [54, 63, 72, 55, 64, 73, 56, 65, 74], [57, 66, 75, 58, 67, 76, 59, 68, 77], [60, 69, 78, 61, 70, 79, 62, 71, 80]];

fn main() {
    /* 
    let mut num_false = 0;
    for n in 1..5000 {
        let mut state = read_sudoku(n);
        let mut candidates = init_candidates(&state);

        //display(&state);
        let mut i = 0;
        while !check_finish(&state) && i < 50 {
            i += 1;
            solve_single_candidates(&mut state, &mut candidates);
            solve_single_possibility(&mut state, &mut candidates);
            solve_combined_candidates(&mut candidates);
        }
        //println!("Iterations: {}, Correct: {}", i, check_correct(&state));
        //display(&state);
        if !check_correct(&state) {
            num_false += 1;
        }
    }
    println!("{}", num_false);
*/

    let mut state = read_sudoku(9);
    let mut candidates = init_candidates(&state);

    display(&state);
    
    let mut i = 0;
    
    
    while !check_finish(&state) && i < 50 {
        i += 1;
        solve_single_candidates(&mut state, &mut candidates);
        solve_single_possibility(&mut state, &mut candidates);
        solve_combined_candidates(&mut candidates);
        
    }
    

    println!("Iterations: {}, Correct: {}", i, check_correct(&state));
    display(&state);
    display_candidates(&candidates);

}

/*
    ----- inits -----
*/

fn read_sudoku(num: usize) -> [u8; 81] {
    let file = File::open("all_17_clue_sudokus.txt").expect("Could not open file");
    let line = BufReader::new(file)
        .lines()
        .nth(num)
        .expect("No such line")
        .expect("Could not read line");
    let mut state = [0u8; 81];
    

    for (i, char) in line.chars().enumerate() {
        state[i] = (char as u32 - '0' as u32) as u8;
    }
    state
}


fn init_candidates(state: &[u8]) -> [u16; 81] {
    let mut candidates: [u16; 81] = [0b11_1111_1110; 81];
    for i in 0..81 {
        if state[i] != 0 {
            let (row, col) = index_to_coords(i);
            remove_candidates(&mut candidates, row, col, state[i]);
        }
    }
    candidates
}



/*
    ----- Display ----
 */

fn display(state: &[u8]) {
    for i in 0..9{
        for j in 0..9{
            if j == 3 || j == 6 {
                print!("| ");
            }
            print!("{} ", state[9*i+j]);
            
        }
        println!("");
        if i == 2 || i == 5 {
            println!("---------------------");
        }
    }
    println!("");
}


fn display_candidates(candidates: &[u16; 81]) {
    for i in 0..9 {
        for j in 0..9 {
            if j == 3 || j == 6 {
                print!(" | ");
            }
            let candidate = candidate_to_vektor(candidates[coords_to_index(i, j)]);
            let cell: String;
            if candidate == [0] {
                cell = format!("[SET]");
            } else {
                cell = format!("{:?}", candidate);
            }
            print!("{:<20}", cell);
        }
        println!();
        if i == 2 || i == 5 {
            println!("{}", "-".repeat(3 * 20 + 3 + 3 * 20 + 3 + 3 * 20 + 4));
        }
    }
    println!();
}



/*
    ----- State funcions ----
*/

fn get_field(state: &[i8], row: usize, col: usize) -> i8{
    state[row * 9 + col]
}


fn set_field(state: &mut [u8; 81], row: usize, col: usize, value: u8) {
    state[row * 9 + col] = value;
}



/*
    ----- candidate functions ------
*/
fn candidate_to_number(candidate: &u16) -> u8 {
    candidate.trailing_zeros() as u8
}


fn add_candidate(candidates: &mut [u16; 81], index: usize, value: u8) {
    let candidate_set = &mut candidates[index];
    *candidate_set |= 1 << value;
}


fn remove_candidate(candidates: &mut [u16; 81], index: usize, mask: u16) {
    let candidate_set = &mut candidates[index];
    *candidate_set &= !mask;
}


fn candidate_to_vektor(mut candidate: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(candidate.count_ones() as usize);
    while candidate != 0 {
        out.push(candidate.trailing_zeros() as u8);
        candidate &= candidate - 1;
    }
    out
}


fn remove_candidates(candidates: &mut [u16; 81], row: usize, col: usize, value: u8) {
    candidates[coords_to_index(row, col)] = 0b_00_0000_0001;
    
    let mask: u16 = 1 << value;
    let blk_indicies = BLK_LIST[get_blk_index_by_coords(row, col)];
    for i in 0..9 {
        remove_candidate(candidates, coords_to_index(i, col), mask);
        remove_candidate(candidates, coords_to_index(row, i), mask);
        remove_candidate(candidates, blk_indicies[i], mask);
    }
}

fn remove_candidates_mask(candidates: &mut [u16; 81], row: usize, col: usize, mask: u16) {
    candidates[coords_to_index(row, col)] = 0b_00_0000_0001;
    
    let blk_indicies = BLK_LIST[get_blk_index_by_coords(row, col)];
    for i in 0..9 {
        remove_candidate(candidates, coords_to_index(i, col), mask);
        remove_candidate(candidates, coords_to_index(row, i), mask);
        remove_candidate(candidates, blk_indicies[i], mask);
    }
}



/*
    ----- helper functions 
*/

fn get_blk_index_by_coords(row: usize, col: usize) -> usize {
    (col / 3) + ((row/3) * 3)
}

fn get_blk_index_by_index(index: usize) -> usize {
    (index % 9 / 3) + (index / 27 * 3)
}


fn coords_to_index(row: usize, col: usize) -> usize{
    row * 9 + col
}


fn index_to_coords(index: usize) -> (usize, usize) {
    let col = index % 9;
    let row = (index - col)/9;
    (row, col)
}



//TODO: think about this
fn get_row<T: Copy>(table: &[T; 81], row: usize) -> [T; 9] {
    let start = row * 9;
    std::array::from_fn(|i| table[start + i])
}


fn get_col<T: Copy>(table: &[T; 81], col: usize) -> [T; 9] {
    std::array::from_fn(|i| table[col + i * 9])
}


fn get_blk<T: Copy>(table: &[T; 81], index: usize) -> [T; 9] {
    let start = (index / 3) * 27 + (index % 3) * 3;
    std::array::from_fn(|i| table[start + (i / 3) * 9 + (i % 3)])
}



/*
    ----- solving functions -----
*/

fn solve_single_candidates(state: &mut [u8; 81], candidates: &mut [u16; 81]) { 
    for index in 0usize..81usize {
        let candidate = candidates[index];

        if candidate & 1 != 1 && candidate.count_ones() == 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
            state[index] = candidate_to_number(&candidate);
            let (row, col) = index_to_coords(index);
            remove_candidates_mask(candidates, row, col, candidate);

        }

    }
} 


fn solve_single_possibility(state: &mut [u8; 81], candidates: &mut [u16; 81]) {
    // rows
    for row in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in ROW_LIST[row] {
            let candidate: u16 = candidates[index];
            if candidate & 1 != 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & candidate;
                appeared_once_buffer |= candidate;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in ROW_LIST[row] {
            let candidate: u16 = candidates[index];
            if (candidate&only_once_mask).count_ones() == 1 { 
                let col = index % 9;
                remove_candidates_mask(candidates, row, col, candidate&only_once_mask);
                set_field(state, row, col, candidate_to_number(&(candidate&only_once_mask)));
            }
        }
    }
    // cols
    for col in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in COL_LIST[col] {
            let candidate: u16 = candidates[index];
            if candidate & 1 != 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & candidate;
                appeared_once_buffer |= candidate;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in COL_LIST[col] {
            let candidate: u16 = candidates[index];
            if (candidate&only_once_mask).count_ones() == 1 { 
                let row = (index - col)/9;
                remove_candidates_mask(candidates, row, col, candidate&only_once_mask);
                set_field(state, row, col, candidate_to_number(&(candidate&only_once_mask)));
            }
        }
    }
    // blks
    for blk in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in BLK_LIST[blk] {
            let candidate: u16 = candidates[index];
            if candidate & 1 != 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & candidate;
                appeared_once_buffer |= candidate;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in BLK_LIST[blk] {
            let candidate: u16 = candidates[index];
            if (candidate&only_once_mask).count_ones() == 1 { 
                let (row, col) = index_to_coords(index);
                remove_candidates_mask(candidates, row, col, candidate&only_once_mask);
                set_field(state, row, col, candidate_to_number(&(candidate&only_once_mask)));
            }
        }
    }
}



fn get_combined(candidates: &[u16;9]) -> [u16; 3] {
    let mut combined_candidates: [u16; 3] = [0u16; 3];
    for i in 0..3 {
        let mut combined_candidate = 0b00_0000_0000;
        for j in 0..3 {
            let candidate = candidates[i*3+j];
            if candidate & 1 != 1 {
                combined_candidate |= candidate;
            }
        }
        combined_candidates[i] = combined_candidate;
    }
    combined_candidates
}

fn better_xor(arr: &[u16;3]) -> u16 {
    let a = arr[0];
    let b = arr[1];
    let c = arr[2];
    let at_least_one = a | b | c;
    let two_or_more  = (a & b) | (b & c) | (a & c);
    at_least_one & !two_or_more
}
// find and remove candidates which are needed in a different structure
fn solve_combined_candidates(candidates: &mut [u16; 81]) {
    // iterate structures
    for i in 0..9 {
        // blocks horizontal patterns

        // get candidate list and combine
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in BLK_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = candidates[*global_index]
        }
        let combined_candidates = get_combined(&candidate_lists);
        let mut singular_candidate = better_xor(&combined_candidates);

        
        
        
        // find candidates which occur only in one row
        while singular_candidate != 0 {
            // get affected number
            let num = singular_candidate.trailing_zeros() as u16;
            let mask: u16 = 1 << num;

            // find location of affected number by searching through the three combined candidates
            for row in 0..3 {
                if (combined_candidates[row] >> num) & 1 != 0{
                    // remove candidates
                    for index in ROW_LIST[(i/3)*3 + row] {
                        if !(BLK_LIST[i].contains(&index)) {
                            remove_candidate(candidates, index, mask);
                            
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }

        // blocks vertical patterns

        // get candidate list and combine
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in BLK_LIST_VER[i].iter().enumerate() {
            candidate_lists[local_index] = candidates[*global_index]
        }        
        let combined_candidates = get_combined(&candidate_lists);
        let mut singular_candidate = better_xor(&combined_candidates);

        
        // find candidates which occur only in one col
        while singular_candidate != 0 {
            // get affected number
            let num = singular_candidate.trailing_zeros() as u16;
            let mask: u16 = 1 << num;

            // find location of affected number by searching through the three combined candidates
            for col in 0..3 {
                if (combined_candidates[col] >> num) & 1 != 0{
                    // remove candidates
                    for index in COL_LIST[(i%3)*3 + col] {
                        if !(BLK_LIST[i].contains(&index)) {
                            remove_candidate(candidates, index, mask);
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }

        // patterns in rows


        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in ROW_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = candidates[*global_index]
        }
        let combined_candidates = get_combined(&candidate_lists);
        let mut singular_candidate = better_xor(&combined_candidates);
        
        // find candidates which occur only in one block
        while singular_candidate != 0 {
            // get affected number
            let num = singular_candidate.trailing_zeros() as u16;
            let mask: u16 = 1 << num;

            // find location of affected number by searching through the three combined candidates
            for blk in 0..3 {
                if (combined_candidates[blk] >> num) & 1 != 0{
                    // remove candidates
                    for index in BLK_LIST[(i/3)*3 + blk] {
                        if !(ROW_LIST[i].contains(&index)) {
                            remove_candidate(candidates, index, mask);
                            
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }

        // patterns in cols

        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in COL_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = candidates[*global_index]
        }
        let combined_candidates = get_combined(&candidate_lists);
        let mut singular_candidate = better_xor(&combined_candidates);
        
        // find candidates which occur only in one block
        while singular_candidate != 0 {
            // get affected number
            let num = singular_candidate.trailing_zeros() as u16;
            let mask: u16 = 1 << num;

            // find location of affected number by searching through the three combined candidates
            for blk in 0..3 {
                if (combined_candidates[blk] >> num) & 1 != 0{
                    // remove candidates
                    for index in BLK_LIST[i/3 + blk * 3] {
                        if !(COL_LIST[i].contains(&index)) {
                            remove_candidate(candidates, index, mask);
                            
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }


        

    }
}


/*
    ----- checking functions -----
*/

fn check_finish(state: &[u8; 81]) -> bool {
    for i in 0..81 {
        if state[i] == 0 {
            return false;
        }
    }
    true
}


fn check_correct(state: &[u8; 81]) -> bool {
    let set: HashSet<u8> = (1..=9).collect();
    for i in 0..9 {
        let row = get_row(state, i);
        let row_set: HashSet<u8> = row.into_iter().collect();
        if row_set != set {
            return false;
        }

        let col = get_col(state, i);
        let col_set: HashSet<u8> = col.into_iter().collect();
        if col_set != set {
            return false;
        }

        let blk = get_blk(state, i);
        let blk_set: HashSet<u8> = blk.into_iter().collect();
        if blk_set != set {
            return false;
        }
    }
    true
}
