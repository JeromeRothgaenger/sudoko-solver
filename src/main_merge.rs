use std::fs::File;

use std::io::{BufRead, BufReader};

use std::time::Instant;


const CHECKING_NUM: u16 = 1024;

const ROW_LIST: [[usize; 9]; 9]     = [[0, 1, 2, 3, 4, 5, 6, 7, 8], [9, 10, 11, 12, 13, 14, 15, 16, 17], [18, 19, 20, 21, 22, 23, 24, 25, 26], [27, 28, 29, 30, 31, 32, 33, 34, 35], [36, 37, 38, 39, 40, 41, 42, 43, 44], [45, 46, 47, 48, 49, 50, 51, 52, 53], [54, 55, 56, 57, 58, 59, 60, 61, 62], [63, 64, 65, 66, 67, 68, 69, 70, 71], [72, 73, 74, 75, 76, 77, 78, 79, 80]];
const COL_LIST: [[usize; 9]; 9]     = [[0, 9, 18, 27, 36, 45, 54, 63, 72], [1, 10, 19, 28, 37, 46, 55, 64, 73], [2, 11, 20, 29, 38, 47, 56, 65, 74], [3, 12, 21, 30, 39, 48, 57, 66, 75], [4, 13, 22, 31, 40, 49, 58, 67, 76], [5, 14, 23, 32, 41, 50, 59, 68, 77], [6, 15, 24, 33, 42, 51, 60, 69, 78], [7, 16, 25, 34, 43, 52, 61, 70, 79], [8, 17, 26, 35, 44, 53, 62, 71, 80]];
const BLK_LIST: [[usize; 9]; 9]     = [[0, 1, 2, 9, 10, 11, 18, 19, 20], [3, 4, 5, 12, 13, 14, 21, 22, 23], [6, 7, 8, 15, 16, 17, 24, 25, 26], [27, 28, 29, 36, 37, 38, 45, 46, 47], [30, 31, 32, 39, 40, 41, 48, 49, 50], [33, 34, 35, 42, 43, 44, 51, 52, 53], [54, 55, 56, 63, 64, 65, 72, 73, 74], [57, 58, 59, 66, 67, 68, 75, 76, 77], [60, 61, 62, 69, 70, 71, 78, 79, 80]];
const BLK_LIST_VER: [[usize; 9]; 9] = [[0, 9, 18, 1, 10, 19, 2, 11, 20], [3, 12, 21, 4, 13, 22, 5, 14, 23], [6, 15, 24, 7, 16, 25, 8, 17, 26], [27, 36, 45, 28, 37, 46, 29, 38, 47], [30, 39, 48, 31, 40, 49, 32, 41, 50], [33, 42, 51, 34, 43, 52, 35, 44, 53], [54, 63, 72, 55, 64, 73, 56, 65, 74], [57, 66, 75, 58, 67, 76, 59, 68, 77], [60, 69, 78, 61, 70, 79, 62, 71, 80]];

fn main() {
    let start = Instant::now();

    let mut num_false: u32 = 0;
    let puzzle_amount = 1000;
    for n in 1..1 + puzzle_amount {
        let mut state = read_sudoku(n);

        let mut iteration = 0;
        while !check_finish(&state) && iteration < 20 {
            iteration += 1;
            solve_orphans(&mut state);
            
            solve_prisoners(&mut state);
            
            elimnate_outsiders(&mut state);
            // display_candidates(&state);
            eliminate_foreigners(&mut state);
            // display_candidates(&state);
            eliminate_pagans(&mut state);
            
        }
        //display(&state);
        if !check_correct(&state) {
            num_false += 1;
            // println!("correct: false");
            // display_fields(&state);
        } else {
            // println!("Iterations: {}, correct: true", iteration);
        }
        // display_fields(&state);
    }
    let duration = start.elapsed().as_millis();
    println!("Time elapsed: {:?}s, avg time: {}ms", duration/1000, duration/ puzzle_amount as u128);
    println!("total:{}; false: {}", puzzle_amount, num_false);
}

/*
    ----- inits -----
*/

fn read_sudoku(num: usize) -> [u16; 81] {
    let file = File::open("all_17_clue_sudokus.txt").expect("Could not open file");
    let line = BufReader::new(file)
        .lines()
        .nth(num)
        .expect("No such line")
        .expect("Could not read line");

    let mut state = [0b_011_1111_1110; 81];
    for (i, c) in line.chars().enumerate() {
        if c != '0' {
            set_field(&mut state, i, 1 << (c as u8 - b'0'));
        }
    }

    state
}


/*
    ----- Display ----
 */

fn display_fields(state: &[u16]) {
    for i in 0..9{
        for j in 0..9{
            if j == 3 || j == 6 {
                print!("| ");
            }
            let field = state[9*i+j];
            if field > CHECKING_NUM {
                print!("{} ", field_to_number(&field))
            } else {
                print!("0 ")
            }

            
        }
        println!("");
        if i == 2 || i == 5 {
            println!("---------------------");
        }
    }
    println!("");
}


fn display_candidates(state: &[u16; 81]) {
    for i in 0..9 {
        for j in 0..9 {
            if j == 3 || j == 6 {
                print!(" | ");
            }

            let cell_string: String;
            let field = state[9*i+j];

            if field > CHECKING_NUM {
                cell_string = format!("({})", field_to_number(&field));
            } else {
                cell_string = format!("{:?}", field_to_vector(field));
            }

            print!("{:<20}", cell_string);
        }
        println!();
        if i == 2 || i == 5 {
            println!("{}", "-".repeat(3 * 20 + 3 + 3 * 20 + 3 + 3 * 20 + 4));
        }
    }
    println!();
}



/*
    ----- state funcions - values ----
*/

fn is_field_set_index(state: &[u16; 81], index: usize) -> bool {
    state[index] > CHECKING_NUM
}

fn is_field_set_coords(state: &[u16; 81], row: usize, col: usize) -> bool {
    state[row*9 + col] > CHECKING_NUM
}

fn is_field_set_value(field: u16) -> bool {
    field > CHECKING_NUM
}

fn get_field(state: &[u16], row: usize, col: usize) -> &u16{
    &state[row * 9 + col]
}


fn set_field(state: &mut [u16; 81], index: usize, mut mask: u16) {
    let blk_indicies = BLK_LIST[get_blk_index_by_index(index)];
    let row_start = (index / 9) * 9;
    let col_start = index % 9;
    for i in 0..9 {
        remove_candidate(state, row_start + i, mask);
        remove_candidate(state, col_start + 9*i, mask);
        remove_candidate(state, blk_indicies[i], mask);
    }

    mask |= CHECKING_NUM;
    state[index] = mask | CHECKING_NUM;
}


fn field_to_number(candidate: &u16) -> u8 {
    candidate.trailing_zeros() as u8
}



/*
    ----- state functions - candidates ------
*/

fn field_to_vector(mut candidate: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(candidate.count_ones() as usize);
    while candidate != 0 {
        out.push(candidate.trailing_zeros() as u8);
        candidate &= candidate - 1;
    }
    out
}


fn add_candidate(candidates: &mut [u16; 81], index: usize, value: u8) {
    let candidate_set = &mut candidates[index];
    *candidate_set |= 1 << value;
}


fn remove_candidate(candidates: &mut [u16; 81], index: usize, mask: u16) {
    let candidate_set = &mut candidates[index];
    *candidate_set &= !mask;
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

fn solve_orphans(state: &mut [u16; 81]) { 
    for index in 0usize..81usize {
        let field = state[index];

        if field < CHECKING_NUM && field.count_ones() == 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
            set_field(state, index, state[index]);
        }
    }
} 


fn solve_prisoners(state: &mut [u16; 81]) {
    // rows
    for row in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in ROW_LIST[row] {
            let field: u16 = state[index];
            if field < CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in ROW_LIST[row] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                set_field(state, index, field&only_once_mask);
            }
        }
    }
    // cols
    for col in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in COL_LIST[col] {
            let field: u16 = state[index];
            if field < CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in COL_LIST[col] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                set_field(state, index, field&only_once_mask);
            }
        }
    }
    // blks
    for blk in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in BLK_LIST[blk] {
            let field: u16 = state[index];
            if field < CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in BLK_LIST[blk] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                set_field(state, index, field&only_once_mask);
            }
        }
    }
}


fn get_combined(fields: &[u16; 9]) -> [u16; 3] {
    let mut combined_fields: [u16; 3] = [0u16; 3];
    for i in 0..3 {
        let mut combined_field: u16 = 0b00_0000_0000;
        for j in 0..3 {
            let field = fields[i*3+j];
            if field < CHECKING_NUM {
                combined_field |= field;
            }
        }
        combined_fields[i] = combined_field;
    }
    combined_fields
}

fn better_xor(arr: &[u16; 3]) -> u16 {
    let a = arr[0];
    let b = arr[1];
    let c = arr[2];
    let at_least_one = a | b | c;
    let two_or_more  = (a & b) | (b & c) | (a & c);
    at_least_one & !two_or_more
}
// find and remove candidates which are needed in a different structure
fn elimnate_outsiders(state: &mut [u16; 81]) {
    
    // blocks horizontal patterns
    for i in 0..9 {

        // get candidate list and combine
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in BLK_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = state[*global_index]
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
                            remove_candidate(state, index, mask);
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }
    }

    // blocks vertical patterns
    for i in 0..9 {
        // get candidate list and combine
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in BLK_LIST_VER[i].iter().enumerate() {
            candidate_lists[local_index] = state[*global_index]
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
                            remove_candidate(state, index, mask);
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }
    }
    
    // patterns in rows
    for i in 0..9 {
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in ROW_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = state[*global_index]
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
                            remove_candidate(state, index, mask);
                            
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }
    }
    
    // patterns in cols
    for i in 0..9 {
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in COL_LIST[i].iter().enumerate() {
            candidate_lists[local_index] = state[*global_index]
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
                            remove_candidate(state, index, mask);
                            
                        }
                    }
                }
            }
            singular_candidate &= singular_candidate - 1;
        }
    }
}



fn eliminate_pagans_structure (state: &mut [u16; 81], coord_list: [[usize; 9]; 9]) {
    for i in 0..9 {
        let struct_coords = coord_list[i];
        let mut skip_list: Vec<usize> = vec![];
        let candidate_list: Vec<u16> = struct_coords.iter().map(|&i| state[i]).collect();


        // add set fields to skip list
        for (index, candidate) in candidate_list.iter().enumerate() {
            if *candidate > CHECKING_NUM {
                skip_list.push(index);
            }
        }
        // iterate start positions
        for start in 0..9 {
            // skip already checked fields
            if !skip_list.contains(&start) {
                skip_list.push(start);
                let mut combination = candidate_list[start];
                let mut index = start;
                let mut combined_fields: Vec<usize> = vec![start];

                // create combintions
                while skip_list.len() < 9 && index <= 8 {
                    if skip_list.contains(&index) {
                        index += 1;
                        continue;
                    }
                    let tmp_combination = combination | candidate_list[index];
                    // combine
                    if (tmp_combination ^ combination).count_ones() <= 1 {
                        combination = tmp_combination;
                        skip_list.push(index);
                        combined_fields.push(index);
                        index = start; // optimierbar
                    } else {
                        index += 1;
                    }
                }
                if combination.count_ones() <= combined_fields.len() as u32 {
                    // iterate fields not contained in group
                    for del in 0..9 {
                        if !combined_fields.contains(&del) {
                            state[struct_coords[del]] = (state[struct_coords[del]] | combination)^combination;
                        }
                    }
                }
            }

        }
    }
}

fn eliminate_pagans(state: &mut [u16; 81]) {
    // rows
    eliminate_pagans_structure(state, ROW_LIST);
    eliminate_pagans_structure(state, COL_LIST);
    eliminate_pagans_structure(state, BLK_LIST);   
}

const MASK: u16 = 0x03FE; // 0b0000_0011_1111_1110
const NUM_COLS: usize = 9;

/// Stack `words` as rows, reading only bits 1–9 vertically.
/// `cols[0]` = column of bit-1, `cols[8]` = column of bit-9.
fn read_vertically_u16(words: &[u16]) -> [u16; NUM_COLS] {
    assert!(words.len() <= 64, "too many words: columns won't fit in u64");

    let n = words.len();
    let mut cols = [0u16; NUM_COLS];

    for (row, &word) in words.iter().enumerate() {
        let word = word & MASK; // mask off bit 0 and bits 10-15
        for col in 0..NUM_COLS {
            let bit = (word >> (col as u16 + 1)) & 1; // col 0 = bit 1, col 8 = bit 9
            cols[col] |= (bit as u16) << (n - 1 - row);
        }
    }

    cols
}

fn eliminate_foreigners_structure(state: &mut [u16; 81], coord_list: [[usize; 9]; 9]) {
    for i in 0..9 {
        let struct_coords = coord_list[i];
        let mut skip_list: Vec<usize> = vec![];
        let candidate_list: [u16; 9] = std::array::from_fn(|i| state[struct_coords[i]]);

        let positions_list = read_vertically_u16(&candidate_list);


        // iterate candidates
        for start_candidate in 1..10 {
            // skip already checked candidates
            if !skip_list.contains(&start_candidate) {
                let mut skip_list_copy = skip_list.clone();
                skip_list_copy.push(start_candidate);
                let mut combined_positions = positions_list[start_candidate - 1];
                let mut candidate = start_candidate;
                let mut combined_candidates: u16 = 1 << start_candidate;

                // create combinations
                while skip_list_copy.len() < 9 && candidate <= 9 && combined_candidates.count_ones() < combined_positions.count_ones(){
                    if skip_list_copy.contains(&candidate) {
                        candidate += 1;
                        continue;
                    }
                    let tmp_combination = combined_positions | positions_list[candidate - 1];
                    // combine
                    if (tmp_combination ^ combined_positions).count_ones() <= 1 {
                        combined_positions = tmp_combination;
                        skip_list_copy.push(candidate);
                        combined_candidates |= 1 << candidate;
                        candidate = start_candidate; // optimierbar
                    } else {
                        candidate += 1;
                    }
                }
                if combined_candidates.count_ones() == combined_positions.count_ones() as u32 && combined_candidates.count_ones() > 1 {
                    // find fields where foreigners need to be eliminated
                    let mut positions = Vec::with_capacity(combined_positions.count_ones() as usize);
                    while combined_positions != 0 {
                        positions.push(8 - (combined_positions.trailing_zeros()) as usize);
                        combined_positions &= combined_positions - 1;
                    }
                    
                    // iterate candidates
                    for position in positions {
                        state[struct_coords[position]] = state[struct_coords[position]] & combined_candidates;
                    }
                }
            }

        }
    }
}

fn eliminate_foreigners(state: &mut [u16; 81]) {
    // rows
    eliminate_foreigners_structure(state, ROW_LIST);
    eliminate_foreigners_structure(state, COL_LIST);
    eliminate_foreigners_structure(state, BLK_LIST);   
}

/*
fn solver(state: &mut [u8; 81], candidates: &mut [u16; 81], i: &mut u32) -> bool {
    let mut prev_state = *state;
    let mut state_stack: Vec<([u16; 81], (u8, u8, u8))> = Vec::new();

    while !check_finish(state) && *i < 50  && !check_conflict(candidates) {
        prev_state = *state;
        *i += 1;
        solve_orphans(state);
        solve_prisoners(state);
        elimnate_outsiders(candidates);
    }
    if check_conflict(candidates) {
        return false;
    }
    return true;
}
*/
/*
    ----- checking functions -----
*/

fn check_finish(state: &[u16; 81]) -> bool {
    for i in 0..81 {
        if state[i] < CHECKING_NUM || state[i].count_ones() != 2 {
            return false;
        }
    }
    true
}

fn check_conflict(state: &[u16; 81]) -> bool{
    for field in state {
        if *field == 0 || *field == CHECKING_NUM {
            return true;
        }
    }
    return false;
}


fn check_correct(state: &[u16; 81]) -> bool {
    check_finish(state) && {
        for row in ROW_LIST {
            let mut buffer: u16 = 0b000_0000_0000;
            for index in row {
                buffer |= state[index];
            }
            if buffer != 0b111_1111_1110 {
                return false;
            }
        }

        for col in COL_LIST {
            let mut buffer = 0b000_0000_0000;
            for index in col {
                buffer |= state[index];
            }
            if buffer != 0b111_1111_1110 {
                return false;
            }
        }

        for blk in BLK_LIST {
            let mut buffer = 0b000_0000_0000;
            for index in blk {
                buffer |= state[index];
            }
            if buffer!= 0b111_1111_1110 {
                return false;
            }
        }
        true
    }
}