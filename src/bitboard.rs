use bitintr;
use bitintr::Popcnt;
use bitintr::Pdep;
use bitintr::Lzcnt;

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

// Win table for all 3x3 boards
// (Geng, 2020)
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

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            x_occupancy: 0,
            o_occupancy: 0,
            to_move: 1,
            next_legal: CELL_MASK,
        }
    }

    fn update_occupancy(&mut self, mut occup: u128, m: u128, block_num: u64) -> u128 {
        //println!("block_num: {}", block_num);
        //println!("block_offset: {}", block_offset);
        
        occup |= m;

        let block = (occup >> (block_num * 9)) & 0x1FF;
        //println!("block: {:#011b}", block);
        let mut level1_win = WIN_TABLE[block as usize / 64] & (1 << (block % 64)); 
        if level1_win != 0 {
            level1_win = 1;
        }
        //println!("level1_win: {}", level1_win);
        occup |=  (level1_win as u128) << (81 + block_num);

        let board = (occup >> 81) & 0x1FF;
        let mut level2_win =
        (WIN_TABLE[board as usize / 64] & (1 << (board % 64))) as u128;
        if level2_win != 0 {
            level2_win = 1;
        }
        occup |= level2_win << 90;

        return occup;
    }
    
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn make_move(&mut self, m: u128) -> bool {
        if m & self.next_legal == 0 {
            return false;
        }
        let mut leading_zeros : u64;
        let m_lower_half: u64 = (m & ((1 << 64) - 1)) as u64;
        let m_upper_half: u64 = (m >> 64) as u64;
        unsafe {
            if m_upper_half == 0 {
                leading_zeros = m_lower_half.lzcnt();
                leading_zeros += 64;
            } else {
                leading_zeros = m_upper_half.lzcnt();
            }
        }
        let space = 127 - leading_zeros;
        let block_num = space / 9;
        let block_offset = space % 9;
        match self.to_move {
            1 => self.x_occupancy = self.update_occupancy(self.x_occupancy, m, block_num),
            -1 => self.o_occupancy = self.update_occupancy(self.o_occupancy, m, block_num),
            _ => panic!("to_move: {}", self.to_move),
        };
        if self.x_occupancy & (1 << (81 + block_offset)) == 0
           && self.o_occupancy & (1 << (81 + block_offset)) == 0
           && (((self.x_occupancy | self.o_occupancy) 
           >> (block_offset * 9)) & 0x1FF) != 0x1FF
                {
            self.next_legal = (0x1FF as u128) << (block_offset * 9);
            //println!("self.next_legal: {:#0130b}", self.next_legal);
        } else {
            self.next_legal = CELL_MASK;
            for i in 0..9 {
                if self.x_occupancy & (1 << (81 + i)) != 0 ||
                self.o_occupancy & (1 << (81 + i)) != 0  {
                    self.next_legal &= !(0x1FF << (i*9));
                }
            }
        }

        self.next_legal &= !self.x_occupancy;
        self.next_legal &= !self.o_occupancy;


        //println!("self.next_legal: {:#0130b}", self.next_legal);

        self.to_move = -self.to_move;
        return true;
    }

    pub fn get_winner(&self) -> i8 {
        if self.x_occupancy & WINNER_MASK != 0 {
            return 1;
        }
        if self.o_occupancy & WINNER_MASK != 0 {
            return -1;
        }
        if (((self.o_occupancy | self.x_occupancy) >> 81) & 0x1FF) == 0x1FF {
            // draw
            return -2;
        }
        return 0;
    }

    pub fn get_to_move(&self) -> i8 {
        return self.to_move;
    }

    pub fn get_moves(&self) -> u128 {
        if self.get_winner() != 0 {
            return 0;
        }
        return self.next_legal;
    }

    pub fn pretty_print(&self) {

    }

    pub fn mask_to_sf(mask: u128) -> u64 {
        let mut leading_zeros: u64;
        let m_lower_half: u64 = (mask & ((1 << 64) - 1)) as u64;
        let m_upper_half: u64 = (mask >> 64) as u64;
        unsafe {
            if m_upper_half == 0 {
                leading_zeros = m_lower_half.lzcnt();
                leading_zeros += 64;
            } else {
                leading_zeros = m_upper_half.lzcnt();
            }
        }
        return 127 - leading_zeros;
    }

    pub fn random_move(moves: u128) -> u128 {
        assert!(moves != 0);
         let m_lower_half: u64 = (moves & ((1 << 64) - 1)) as u64;
         let m_upper_half: u64 = (moves >> 64) as u64;
         let mut upper_popcnt: u64;
         let mut lower_popcnt: u64;
         let upper_popcnt = m_upper_half.popcnt();
         let lower_popcnt = m_lower_half.popcnt();
         let total_popcnt = upper_popcnt + lower_popcnt;
         let mut n = rand::random::<u64>() % total_popcnt; 
         if n < lower_popcnt {
             let result = ((1 << n) as u64).pdep(m_lower_half);
             return result as u128;
         } else {
             n -= lower_popcnt;
             let result = ((1 << n) as u64).pdep(m_upper_half);
             return (result as u128) << 64;
         }
    }

    pub fn iterate_moves(moves: u128, fun: &mut dyn FnMut(u128, i64) -> bool) {
         let mut m_lower_half: u64 = (moves & ((1 << 64) - 1)) as u64;
         let mut m_upper_half: u64 = (moves >> 64) as u64;
         while m_lower_half != 0 {
            let mut leading_zeros = m_lower_half.lzcnt();
            m_lower_half &= !(1 << (63 - leading_zeros));
            leading_zeros += 64;
            let next_move = (1 as u128) << (127 - leading_zeros);
            let next_move_fs = (127 - leading_zeros) as i64;
            if !fun(next_move, next_move_fs) {
                return;
            }
         }
         while m_upper_half != 0 {
            let leading_zeros = m_upper_half.lzcnt();
            let next_move = (1 as u128) << (127 - leading_zeros as usize);
            m_upper_half &= !(1 << (63 - leading_zeros));
            let next_move_fs = (127 - leading_zeros) as i64;
            if !fun(next_move, next_move_fs) {
                return;
            }
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
     fn test_basic_moves_2lv_bitboard() {
         let mut b = BitBoard::new();
         let moves = vec![20, 22, 38, 21, 29, 23, 50, 49, 41, 46, 14, 52];
         for _i in moves {
             let i = 1 << _i;
             assert!(b.make_move(i));
             //b.pretty_print();
             println!("move: {}", _i);
         }
         assert!(b.make_move(1 << 68));
         //assert!(!b.make_move(1 << 48));
     }

     #[test]
     fn test_basic_victory_2lv_bitboard() {
         let mut b = BitBoard::new();
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for _i in moves {
             let  i = 1 << _i;
             assert!(b.make_move(i));
             //b.pretty_print();
             println!("move: {}", _i);
         }
         assert!(b.get_winner() == -1);
     }
     /*#[test]
     fn test_undo_basic_victory_2lv_bitboard() {
         let mut b = Board::new(2);
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for i in &moves {
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", i);
         }
         for _i in &moves {
             b.undo_move();
         }
         for i in &moves {
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", i);
         }
         assert!(b.winner == Player::O);
     }*/

     #[test]
     fn test_full_square_ascend_2lv_bitboard() {
         let mut b = BitBoard::new();
         let moves = vec![0, 1, 10, 9, 5, 45, 7, 70, 71, 80, 72, 4, 36, 8, 
                          73, 11, 18, 2, 20, 21, 27, 3, 33, 54, 6, 
                          61, 63, 13];
         for _i in moves {
             let i = 1 << _i;
             assert!(b.make_move(i));
             //b.pretty_print();
             println!("move: {}", _i);
         }
     }

     #[test]
     fn test_draw_2lv_bitboard() {
         let mut b = BitBoard::new();
         let moves = vec![0, 1, 9, 4, 36, 7, 70, 71, 79, 67, 43, 63, 20, 21, 
                          31, 40, 37, 13, 38, 23, 49, 22, 10, 14, 52, 55, 11, 
                          50, 46, 30, 29, 27, 32, 33, 58, 78, 59, 72, 57, 73, 74, 
                          76, 77, 80];
         for _i in moves {
             let i = 1 << _i;
             assert!(b.make_move(i));
            // b.pretty_print();
             println!("move: {}", _i);
         }
         assert!(b.get_winner() == -2);
     }

     /*#[test]
     fn test_basic_undo_2lv_bitboard() {
         let mut b = Board::new(2);
         let moves = vec![20, 22, 38, 21, 29, 23, 50, 49, 41, 46, 14, 52];
         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.make_move(68));
         assert!(!b.make_move(48));

         for _i in 0..(moves.len()+1) {
             assert!(b.undo_move());
         }

         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.make_move(68));
         assert!(!b.make_move(48));
     }*/

     /*#[test]
     fn test_basic_victory_undo_2lv_bitboard() {
         let mut b = Board::new(2);
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14, 47, 22, 38, 21, 29, 23];
         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.winner == Player::O);
         for _i in 0..moves.len() {
             assert!(b.undo_move());
         }

         for i in &moves {
             assert!(b.make_move(*i));
             assert!(b.undo_move());
             assert!(b.make_move(*i));
             b.pretty_print();
             println!("move: {}", *i);
         }
         assert!(b.winner == Player::O);
     }*/

     /*#[test]
     fn test_full_square_ascend_undo_2lv_bitboard() {
         let mut b = Board::new(2);
         let moves = vec![0, 1, 10, 9, 5, 45, 7, 70, 71, 80, 72, 4, 36, 8, 
                          73, 11, 18, 2, 20, 21, 27, 3, 33, 54, 6, 
                          61, 63, 13];
         for i in moves {
             assert!(b.make_move(i));
             assert!(b.undo_move());
             assert!(b.make_move(i));
             b.pretty_print();
             println!("move: {}", i);
         }
     }*/

    fn get_moves_at_depths_bitboard(b: &mut BitBoard, depth: usize, out: &mut Vec<usize>) {
         if depth == 0 {
             return;
         }
         let mut moves = b.get_moves();
         if moves == 0 {
             return;
         }
         let out_len = out.len();
        // out[out_len - depth] += moves.len();
        BitBoard::iterate_moves(moves, &mut |next_move: u128, next_move_sf: i64| {
            out[out_len - depth] += 1;
            let mut next_b = b.clone();
            next_b.make_move(next_move);
            get_moves_at_depths_bitboard(&mut next_b, depth - 1, out);
            return true;
        });
    }

     fn get_moves_at_depths_no_vector_bitboard(b: &mut BitBoard, depth: usize) -> usize {
         if depth == 0 {
             return 1;
         }
         let moves = b.get_moves();
         let mut sum = 0;
         for i in 0..81 {
             if ((1 << i) & moves) != 0 {
                let mut next_b = b.clone();
                assert!(next_b.make_move(1 << i));
                sum += get_moves_at_depths_no_vector_bitboard(&mut next_b, depth - 1);
             }
         }
         return sum;
     }

     /*fn get_moves_at_depths_undo(b: &mut Board, depth: usize, out: &mut Vec<usize>) {
         let moves = b.get_moves();
         let out_len = out.len();
         out[out_len - depth] += moves.len();
         for m in &moves {
             let temp = out[out_len - 1];
             if !b.make_move(*m) {
                 b.pretty_print();
                 println!("{}", *m);
                 println!("{:?}", b.next_legal);
                 println!("{:?}", moves);
                 println!("{:?}", b.get_moves());
                 assert!(false);
             }
             if depth > 1 {
                get_moves_at_depths_undo(b, depth - 1, out);
             }
             assert!(b.undo_move());
             if depth == out_len  {
                //println!("{}: {}", *m, out[out_len - 1] - temp);
             }
             if moves != b.get_moves() {
                 println!("After undo: {:?}", b.get_moves());
                 println!("Before undo: {:?}", moves);
                 assert!(false);
             }
         }
     }*/

     /*fn get_moves_at_depths_thread(other_b: &Board, depth: usize, out: &mut Vec<usize>) {
         if depth == 0 {
             return;
         }
         let b = other_b.clone();
         let moves = b.get_moves();
         let out_len = out.len();
         out[out_len - depth] += moves.len();
         let threads = Vec::new();
         for m in &moves {
             assert!(b.make_move(*m));
             get_moves_at_depths_undo(b, depth - 1, out);
             assert!(b.undo_move());
             if moves != b.get_moves() {
                 println!("After undo: {:?}", b.get_moves());
                 println!("Before undo: {:?}", moves);
                 assert!(false);
             }
         }sdfasdf
     }*/

     #[test]
     #[ignore]
     fn test_move_gen_2lv_bitboard() {
         let mut b = BitBoard::new();
         let depth = 8; // actually depth + 1
         println!("Level\tMoves");
         let mut levels = Vec::new();
         for _i in 0..depth {
             levels.push(0);
         }
         let mut now = Instant::now();
         let moves = vec![0, 3, 27, 4, 36, 5, 46, 13, 37, 12, 28, 14];
        for m in moves {
            //b.make_move(m);
        }
         get_moves_at_depths_bitboard(&mut b, depth, &mut levels);
         println!("Search took {} seconds", now.elapsed().as_secs());
         for i in 0..depth {
             print!("{}", i);
             print!("\t");
             println!("{}", levels[i]);
         }
         //b = Board::new(2);
         //now = Instant::now();
         //let lowest_depth_moves = get_moves_at_depths_no_vector_bitboard(&mut b, depth);
        // println!("Search took {} seconds", now.elapsed().as_secs());
         //println!("Level {}: {}", depth - 1, lowest_depth_moves);
     }
}