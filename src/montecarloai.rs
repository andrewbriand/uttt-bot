use crate::ai::AI;

use rand;

use crate::board::Board;
use crate::board::Player;


pub struct MonteCarloAI {
   board: Board,
}

impl AI for MonteCarloAI {
    fn get_move(&mut self, last_move: i64) -> i64 {
        if last_move != -1 {
          self.board.make_move(last_move as usize);
        }
        let result = self.search(100000) as i64;
        self.board.make_move(result as usize);
        return result;
    }
    
    fn cleanup(&mut self) {}
}

impl MonteCarloAI {
    pub fn new() -> MonteCarloAI {
        let result = MonteCarloAI {
            board: Board::new(2),
        };
        return result;
    }

    pub fn search(&mut self, games: usize) -> usize {
        let moves = self.board.get_moves();
        let mut max = -1;
        let mut best_move = 81;
        let games_per_move = games / moves.len();
        for m in moves {
            let mut win_count = 0;
            for _i in 0..games_per_move {
                let mut next_board = self.board.clone();
                next_board.make_move(m);
                let result = self.search_helper(&mut next_board);
                if result == -1 && self.board.get_to_move() == Player::O {
                    win_count += 1;
                } else if result == 1 && self.board.get_to_move() == Player::X {
                    win_count += 1;
                }
            }
            if win_count > max {
                max = win_count;
                best_move = m;
            }
        }
        println!("MCTS: {}", max as f64 / games_per_move as f64);
        return best_move;
    }

    pub fn search_helper(&mut self, board: &mut Board) -> i64 {
        if board.winner == Player::O {
            return -1;
        } else if board.winner == Player::X {
            return 1;
        } else if board.winner == Player::DEAD {
            return 0;
        }
        let moves = board.get_moves();
        let next_move = moves[rand::random::<usize>() % moves.len()];
        board.make_move(next_move);
        let result = self.search_helper(board);
        return result;
    }
}