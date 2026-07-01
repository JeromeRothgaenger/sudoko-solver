use crate::helper;

pub(crate) fn check_finish(state: &[u16; 81]) -> bool {
    for i in 0..81 {
        if state[i] < helper::CHECKING_NUM || state[i].count_ones() != 2 {
            return false;
        }
    }
    true
}

pub(crate) fn check_conflict(state: &[u16; 81]) -> bool{
    for field in state {
        if *field == 0 || *field == helper::CHECKING_NUM {
            return true;
        }
    }
    return false;
}


pub(crate) fn check_correct(state: &[u16; 81]) -> bool {
    check_finish(state) && {
        for row in helper::ROW_LIST {
            let mut buffer: u16 = 0b000_0000_0000;
            for index in row {
                buffer |= state[index];
            }
            if buffer != 0b111_1111_1110 {
                return false;
            }
        }

        for col in helper::COL_LIST {
            let mut buffer = 0b000_0000_0000;
            for index in col {
                buffer |= state[index];
            }
            if buffer != 0b111_1111_1110 {
                return false;
            }
        }

        for blk in helper::BLK_LIST {
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