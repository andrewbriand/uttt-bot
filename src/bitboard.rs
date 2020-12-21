use std::io;
use std::time::{Duration, Instant};
use std::thread;
use core::arch::x86_64::_pdep_u64;
use rand::{SeedableRng, Rng, RngCore};
use rand::rngs::SmallRng;

static WIN_TABLE: [u64; 8] = [
    0xff80808080808080,
    0xfff0aa80faf0aa80,
    0xffcc8080cccc8080,
    0xfffcaa80fefcaa80,
    0xfffaf0f0aaaa8080,
    0xfffafaf0fafaaa80,
    0xfffef0f0eeee8080,
    0xffffffffffffffff,
];

static CELL_MASK: u128 = (1 << 81) - 1;
static WINNER_MASK: u128 = 1 << 90;

// Only supports 2 levels
#[derive(Debug)]
#[derive(Clone)]
pub struct BitBoard {
    pub x_occupancy: u128,
    pub o_occupancy: u128,
    // 1 if X is to move
    // -1 if O is to move
    pub to_move: i8,
    // A bit mask that is 1 in the cells
    // that are possible next moves
    next_legal: u128,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            x_occupancy: 0,
            o_occupancy: 0,
            to_move: 1,
            next_legal: CELL_MASK,
        }
    }

    fn update_occupancy(&mut self, mut occup: u128, m: u128, block_num: u64) -> (u128, bool) {
        //println!("block_num: {}", block_num);
        //println!("block_offset: {}", block_offset);
        
        occup |= m;
        let mut capture = false;

        let block = (occup >> (block_num * 9)) & 0x1FF;
        //println!("block: {:#011b}", block);
        let mut level1_win = WIN_TABLE[block as usize / 64] & (1 << (block % 64)); 
        let ltemp = level1_win != 0;
        capture = ltemp;
        level1_win = ltemp as u64;
        occup |= (ltemp as u128 * 0x1FF) << (block_num * 9);
        /*if level1_win != 0 {
            capture = true;
            level1_win = 1;
            occup |= 0x1FF << (block_num * 9);
        }*/
        //println!("level1_win: {}", level1_win);
        occup |=  (level1_win as u128) << (81 + block_num);
        occup |= (level1_win as u128) << (91 + block_num);

        let cond = block == 0x1FF;
        occup |= (cond as u128) << (91 + block_num);
        /*if block == 0x1FF {
           occup |= (1 << (91 + block_num);
        }*/

        let board = (occup >> 81) & 0x1FF;
        let mut level2_win =
        (WIN_TABLE[board as usize / 64] & (1 << (board % 64))) as u128;
        level2_win = (level2_win != 0) as u128;
        occup |= level2_win << 90;

        return (occup, capture);
    }
    
    //#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn make_move(&mut self, m: u128) -> bool {
        //if m & self.next_legal == 0 {
        //    return false;
       // }
        let mut result = false;
        let mut tuple;
        //let mut leading_zeros : u64 = m.leading_zeros() as u64;
        let space = m.trailing_zeros() as u64;
        let block_num = space / 9;
        let block_offset = space % 9;
        if self.to_move == 1 {
            tuple  = self.update_occupancy(self.x_occupancy, m, block_num); 
            self.x_occupancy = tuple.0; 
            result = tuple.1;
        } else {
            tuple = self.update_occupancy(self.o_occupancy, m, block_num); 
            self.o_occupancy = tuple.0; 
            result = tuple.1;
        }
        /*if (self.x_occupancy | self.o_occupancy) & (1 << (91 + block_offset)) == 0
        /*(self.x_occupancy | self.o_occupancy) & (1 << (81 + block_offset)) == 0
           //&& (((self.x_occupancy | self.o_occupancy) 
           // >> (block_offset * 9)) & 0x1FF) != 0x1FF
           && (self.x_occupancy | self.o_occupancy) 
           & ((0x1FF as u128) << (block_offset * 9)) != ((0x1FF as u128) << (block_offset * 9))
          // (self.x_occupancy | self.o_occupancy) & ((1 << (81 + block_offset)) | ()) == */
        {
            self.next_legal = (0x1FF as u128) << (block_offset * 9);
            //println!("self.next_legal: {:#0130b}", self.next_legal);
        } else {
            self.next_legal = CELL_MASK;
        }*/

        self.next_legal = CELL_MASK * (((self.x_occupancy | self.o_occupancy) & (1 << (91 + block_offset)) != 0) as u128);
        self.next_legal |= (0x1FF as u128) << (block_offset * 9);

        self.next_legal &= !(self.x_occupancy | self.o_occupancy);
        //println!("self.next_legal: {:#0130b}", self.next_legal);
        self.to_move = -self.to_move;
        return result;
    }

    pub fn get_winner(&self) -> i8 {
        if self.x_occupancy & WINNER_MASK != 0 {
            return 1;
        }
        if self.o_occupancy & WINNER_MASK != 0 {
            return -1;
        }
        if self.get_moves() == 0 {
            let o_blocks = ((self.o_occupancy >> 81) & 0x1FF).count_ones();
            let x_blocks = ((self.x_occupancy >> 81) & 0x1FF).count_ones();
            if (o_blocks > x_blocks) {
                return -1;
            } else if (x_blocks > o_blocks) {
                return 1;
            }
            return -2;
        }
        return 0;
    }

    pub fn get_to_move(&self) -> i8 {
        return self.to_move;
    }

    pub fn get_moves(&self) -> u128 {
       // if self.get_winner() != 0 {
        //    return 0;
        //}
        return self.next_legal;
    }

    pub fn pretty_print(&self) {

    }

    pub fn mask_to_sf(mask: u128) -> u64 {
        return mask.trailing_zeros() as u64;
    }

    pub fn random_move(moves: u128, rand: &mut SmallRng) -> u128 {
        //assert!(moves != 0);
         let m_lower_half: u64 = (moves & ((1 << 64) - 1)) as u64;
         let m_upper_half: u64 = (moves >> 64) as u64;
         let upper_popcnt = m_upper_half.count_ones() as u64;
         let lower_popcnt = m_lower_half.count_ones() as u64;
         let total_popcnt = upper_popcnt + lower_popcnt;
         let mut n = (rand.next_u64()) % (total_popcnt as u64); 
         if n < lower_popcnt {
             //let result = ((1 << n) as u64).pdep(m_lower_half);
             unsafe {
              let result = _pdep_u64(((1 as u64) << n) as u64, m_lower_half);
              return result as u128;
             }
            
         } else {
             n -= lower_popcnt;
            // let result = ((1 << n) as u64).pdep(m_upper_half);
            unsafe {
            let result = _pdep_u64(((1 as u64) << n) as u64, m_upper_half);
            return (result as u128) << 64;
            }
             
         }
    }

    pub fn iterate_moves(moves: u128, fun: &mut dyn FnMut(u128, i64) -> bool) {
         //let mut m_lower_half: u64 = (moves & ((1 << 64) - 1)) as u64;
         //let mut m_upper_half: u64 = (moves >> 64) as u64;
         let mut m_mut = moves;
         while m_mut != 0 {
            let trailing_zeros = m_mut.trailing_zeros();
            let next_move = (1 as u128) << (trailing_zeros);
            m_mut &= !next_move;
            let next_move_fs = (trailing_zeros) as i64;
            if !fun(next_move, next_move_fs) {
                return;
            }
         }
        /* return;
         while m_lower_half != 0 {
            let mut leading_zeros = lzcnt(m_lower_half);
            m_lower_half &= !(1 << (63 - leading_zeros));
            leading_zeros += 64;
            let next_move = (1 as u128) << (127 - leading_zeros);
            let next_move_fs = (127 - leading_zeros) as i64;
            if !fun(next_move, next_move_fs) {
                return;
            }
         }
         while m_upper_half != 0 {
            let leading_zeros = lzcnt(m_upper_half);
            let next_move = (1 as u128) << (127 - leading_zeros as usize);
            m_upper_half &= !(1 << (63 - leading_zeros));
            let next_move_fs = (127 - leading_zeros) as i64;
            if !fun(next_move, next_move_fs) {
                return;
            }
         }*/
    }
}
