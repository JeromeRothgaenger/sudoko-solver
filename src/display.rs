use crate::helper;

fn field_to_number(candidate: &u16) -> u8 {
    candidate.trailing_zeros() as u8
}

fn field_to_vector(mut candidate: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(candidate.count_ones() as usize);
    while candidate != 0 {
        out.push(candidate.trailing_zeros() as u8);
        candidate &= candidate - 1;
    }
    out
}

pub(crate) fn display_fields(state: &[u16]) {
    for i in 0..9{
        for j in 0..9{
            if j == 3 || j == 6 {
                print!("| ");
            }
            let field = state[9*i+j];
            if field > helper::CHECKING_NUM {
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


pub(crate) fn display_candidates(state: &[u16; 81]) {
    for i in 0..9 {
        for j in 0..9 {
            if j == 3 || j == 6 {
                print!(" | ");
            }

            let cell_string: String;
            let field = state[9*i+j];

            if field > helper::CHECKING_NUM {
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
