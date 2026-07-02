pub const ROW_LIST: [[usize; 9]; 9]     = [[0, 1, 2, 3, 4, 5, 6, 7, 8], [9, 10, 11, 12, 13, 14, 15, 16, 17], [18, 19, 20, 21, 22, 23, 24, 25, 26], [27, 28, 29, 30, 31, 32, 33, 34, 35], [36, 37, 38, 39, 40, 41, 42, 43, 44], [45, 46, 47, 48, 49, 50, 51, 52, 53], [54, 55, 56, 57, 58, 59, 60, 61, 62], [63, 64, 65, 66, 67, 68, 69, 70, 71], [72, 73, 74, 75, 76, 77, 78, 79, 80]];
pub const COL_LIST: [[usize; 9]; 9]     = [[0, 9, 18, 27, 36, 45, 54, 63, 72], [1, 10, 19, 28, 37, 46, 55, 64, 73], [2, 11, 20, 29, 38, 47, 56, 65, 74], [3, 12, 21, 30, 39, 48, 57, 66, 75], [4, 13, 22, 31, 40, 49, 58, 67, 76], [5, 14, 23, 32, 41, 50, 59, 68, 77], [6, 15, 24, 33, 42, 51, 60, 69, 78], [7, 16, 25, 34, 43, 52, 61, 70, 79], [8, 17, 26, 35, 44, 53, 62, 71, 80]];
pub const BLK_LIST: [[usize; 9]; 9]     = [[0, 1, 2, 9, 10, 11, 18, 19, 20], [3, 4, 5, 12, 13, 14, 21, 22, 23], [6, 7, 8, 15, 16, 17, 24, 25, 26], [27, 28, 29, 36, 37, 38, 45, 46, 47], [30, 31, 32, 39, 40, 41, 48, 49, 50], [33, 34, 35, 42, 43, 44, 51, 52, 53], [54, 55, 56, 63, 64, 65, 72, 73, 74], [57, 58, 59, 66, 67, 68, 75, 76, 77], [60, 61, 62, 69, 70, 71, 78, 79, 80]];
pub const BLK_LIST_VER: [[usize; 9]; 9] = [[0, 9, 18, 1, 10, 19, 2, 11, 20], [3, 12, 21, 4, 13, 22, 5, 14, 23], [6, 15, 24, 7, 16, 25, 8, 17, 26], [27, 36, 45, 28, 37, 46, 29, 38, 47], [30, 39, 48, 31, 40, 49, 32, 41, 50], [33, 42, 51, 34, 43, 52, 35, 44, 53], [54, 63, 72, 55, 64, 73, 56, 65, 74], [57, 66, 75, 58, 67, 76, 59, 68, 77], [60, 69, 78, 61, 70, 79, 62, 71, 80]];

pub const CHECKING_NUM: u16 = 1024;


/*
    ----- state funcions - values ----
*/

pub(crate) fn is_field_set_index(state: &[u16; 81], index: usize) -> bool {
    state[index] > CHECKING_NUM
}

pub(crate) fn is_field_set_coords(state: &[u16; 81], row: usize, col: usize) -> bool {
    state[row*9 + col] > CHECKING_NUM
}

pub(crate) fn is_field_set_value(field: u16) -> bool {
    field > CHECKING_NUM
}

pub(crate) fn get_field(state: &[u16], row: usize, col: usize) -> &u16{
    &state[row * 9 + col]
}


pub(crate) fn set_field(state: &mut [u16; 81], index: usize, mut mask: u16) {
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



/*
    ----- state functions - candidates ------
*/


pub(crate) fn add_candidate(candidates: &mut [u16; 81], index: usize, value: u8) {
    let candidate_set = &mut candidates[index];
    *candidate_set |= 1 << value;
}


pub(crate) fn remove_candidate(candidates: &mut [u16; 81], index: usize, mask: u16) {
    let candidate_set = &mut candidates[index];
    *candidate_set &= !mask;
}



/*
    ----- helper functions 
*/

pub(crate) fn get_blk_index_by_coords(row: usize, col: usize) -> usize {
    (col / 3) + ((row/3) * 3)
}

pub(crate) fn get_blk_index_by_index(index: usize) -> usize {
    (index % 9 / 3) + (index / 27 * 3)
}


pub(crate) fn coords_to_index(row: usize, col: usize) -> usize{
    row * 9 + col
}


pub(crate) fn index_to_coords(index: usize) -> (usize, usize) {
    let col = index % 9;
    let row = (index - col)/9;
    (row, col)
}



//TODO: think about this
pub(crate) fn get_row<T: Copy>(table: &[T; 81], row: usize) -> [T; 9] {
    let start = row * 9;
    std::array::from_fn(|i| table[start + i])
}


pub(crate) fn get_col<T: Copy>(table: &[T; 81], col: usize) -> [T; 9] {
    std::array::from_fn(|i| table[col + i * 9])
}


pub(crate) fn get_blk<T: Copy>(table: &[T; 81], index: usize) -> [T; 9] {
    let start = (index / 3) * 27 + (index % 3) * 3;
    std::array::from_fn(|i| table[start + (i / 3) * 9 + (i % 3)])
}

pub (crate) fn combine_triples(fields: &[u16; 9]) -> [u16; 3] {
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

// combine three inputs, if one and only one is true return true
pub (crate) fn triple_xor(arr: &[u16; 3]) -> u16 {
    let a = arr[0];
    let b = arr[1];
    let c = arr[2];
    let at_least_one = a | b | c;
    let two_or_more  = (a & b) | (b & c) | (a & c);
    at_least_one & !two_or_more
}


pub(crate) fn generate_mask(field: u16, index: u8) -> u16 {
    use core::arch::x86_64::_pdep_u32;
    let chosen = unsafe { _pdep_u32(1 << index, field as u32) } as u16;
    chosen | (1 << 10)
}