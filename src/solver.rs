use crate::checker::check_conflict;
use crate::display::display_candidates;
use crate::display::display_fields;
use crate::helper;
use crate::display;
use crate::checker;
use crate::helper::CHECKING_NUM;

const MAX_ITERATIONS: i16 = 100;


pub(crate) fn solve(state: &mut [u16; 81]) -> i16 {
    let mut iteration :i16 = 0;
    let mut previous_state: [u16; 81] = state.clone();

    let mut result = 0;

    while !checker::check_finish(state) && iteration < MAX_ITERATIONS {
        iteration += 1;
        previous_state = state.clone();

        solve_orphan_prisoners(state);
        if *state != previous_state {continue;}
        eliminate_pagans(state);
        if *state != previous_state {continue;}
        elimnate_outsiders(state);
        if *state != previous_state {continue;}
        eliminate_foreigners(state);
        if *state != previous_state {continue;}

        if check_conflict(state) {
            result = -2;
            display_candidates(state);
            break;
        } else if checker::check_correct(&state) {
            result = iteration;
            break;
        }
        let guesser_result = guesser(state);
        if guesser_result == 0 {
            continue;
        } 
        result += guesser_result;
        break;
    }
    if iteration >= MAX_ITERATIONS {
        println!("Max number of iterations reached: {}", iteration);
        result = -3;
    }
    //display_candidates(state);
    return result;
}

fn solve_without_guesser(state: &mut [u16; 81]) -> i16 {
    let mut iteration :i16 = 0;
    let mut previous_state: [u16; 81] = state.clone();

    let mut result = 0;
    while !checker::check_finish(state) && iteration < MAX_ITERATIONS {
        iteration += 1;
        previous_state = state.clone();

        solve_orphan_prisoners(state);
        if *state != previous_state {continue;}
        eliminate_pagans(state);
        if *state != previous_state {continue;}
        elimnate_outsiders(state);
        if *state != previous_state {continue;}
        eliminate_foreigners(state);
        if *state != previous_state {continue;}

        if check_conflict(state) {
            result = -2;
            break;
        } else if *state == previous_state {
            result = -1;
            break;
        } else if checker::check_correct(&state) {
            result = iteration;
            break;
        }
        if iteration >= MAX_ITERATIONS {
            println!("Max number of iterations reached: {}", iteration);
            result = -3;
        }
    }
    return result;
}

fn solve_orphan_prisoners(state: &mut [u16; 81]) {
    // rows
    for row in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in helper::ROW_LIST[row] {
            let field: u16 = state[index];
            if field < helper::CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;

                if field.count_ones() == 1 { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                    helper::set_field(state, index, state[index]);
                }
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in helper::ROW_LIST[row] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                helper::set_field(state, index, field&only_once_mask);
            }
        }
    }
    // cols
    for col in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in helper::COL_LIST[col] {
            let field: u16 = state[index];
            if field < helper::CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in helper::COL_LIST[col] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                helper::set_field(state, index, field&only_once_mask);
            }
        }
    }
    // blks
    for blk in 0usize..9usize {
        let mut appeared_once_buffer: u16 = 0b_00_0000_0000;
        let mut appeared_multiple_buffer: u16 = 0b_00_0000_0000;
        
        for index in helper::BLK_LIST[blk] {
            let field: u16 = state[index];
            if field < helper::CHECKING_NUM { // Prüfbit nicht gesetzt --> noch keine eingetragene lösung
                appeared_multiple_buffer |= appeared_once_buffer & field;
                appeared_once_buffer |= field;
            }
        }

        let only_once_mask = appeared_once_buffer & !appeared_multiple_buffer;

        for index in helper::BLK_LIST[blk] {
            let field: u16 = state[index];
            if (field&only_once_mask).count_ones() == 1 { 
                helper::set_field(state, index, field&only_once_mask);
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
            if field < helper::CHECKING_NUM {
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
fn elimnate_outsiders(state: &mut [u16; 81]) { // intersection removal and ?pointing?
    
    // blocks horizontal patterns
    for i in 0..9 {

        // get candidate list and combine
        let mut candidate_lists = [0u16; 9];
        for (local_index, global_index) in helper::BLK_LIST[i].iter().enumerate() {
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
                    for index in helper::ROW_LIST[(i/3)*3 + row] {
                        if !(helper::BLK_LIST[i].contains(&index)) {
                            helper::remove_candidate(state, index, mask);
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
        for (local_index, global_index) in helper::BLK_LIST_VER[i].iter().enumerate() {
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
                    for index in helper::COL_LIST[(i%3)*3 + col] {
                        if !(helper::BLK_LIST[i].contains(&index)) {
                            helper::remove_candidate(state, index, mask);
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
        for (local_index, global_index) in helper::ROW_LIST[i].iter().enumerate() {
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
                    for index in helper::BLK_LIST[(i/3)*3 + blk] {
                        if !(helper::ROW_LIST[i].contains(&index)) {
                            helper::remove_candidate(state, index, mask);
                            
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
        for (local_index, global_index) in helper::COL_LIST[i].iter().enumerate() {
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
                    for index in helper::BLK_LIST[i/3 + blk * 3] {
                        if !(helper::COL_LIST[i].contains(&index)) {
                            helper::remove_candidate(state, index, mask);
                            
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
            if *candidate > helper::CHECKING_NUM {
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
    eliminate_pagans_structure(state, helper::ROW_LIST);
    eliminate_pagans_structure(state, helper::COL_LIST);
    eliminate_pagans_structure(state, helper::BLK_LIST);   
}

const MASK: u16 = 0x03FE; // 0b0000_0011_1111_1110
const NUM_COLS: usize = 9;

/// Stack `words` as rows, reading only bits 1–9 vertically.
/// `cols[0]` = column of bit-1, `cols[8]` = column of bit-9.
fn read_vertically_u16(words: &[u16]) -> [u16; NUM_COLS] {
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
        let skip_list: Vec<usize> = vec![];
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
                if combined_candidates >= helper::CHECKING_NUM {
                    println!("hurensohn");
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
                        if state[struct_coords[position]] < helper::CHECKING_NUM {
                            state[struct_coords[position]] = state[struct_coords[position]] & combined_candidates;
                        }
                    }
                }
            }

        }
    }
}

fn eliminate_foreigners(state: &mut [u16; 81]) {
    // rows
    eliminate_foreigners_structure(state, helper::ROW_LIST);
    eliminate_foreigners_structure(state, helper::COL_LIST);
    eliminate_foreigners_structure(state, helper::BLK_LIST);   
}



fn guesser(state: &mut [u16; 81]) -> i16 {
    for (index, field) in state.clone().iter().enumerate() {
        if *field < helper::CHECKING_NUM {
            let mut states_generated = [0u16; 81];  //Buffer where all reached game states are ored together
            
            let base_state = state.clone();
            for candidate in 0..field.count_ones() as u8 {
                let mut state_clone = base_state.clone();
                let mask = helper::generate_mask(*field, candidate);
                helper::set_field(&mut state_clone, index, mask);

                let success = solve_without_guesser(&mut state_clone);
         

                if success >= 0 {
                    *state = state_clone;
                    return success;

                } else if success == -2 {
                    helper::remove_candidate(state, index, mask);

                } else {
                    for fields_generated in 0..81 {
                        states_generated[fields_generated] |= state_clone[fields_generated];
                    }
                }
            }
            for fields_generated in 0..81 {
                state[fields_generated] &= states_generated[fields_generated];
            }
        }
    }
    return 0;
}