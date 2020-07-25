
use crate::ai::AI;
use crate::bitboard::BitBoard;

use std::time::{Duration, Instant};
use std::collections::HashMap;

use rand;

pub struct MCTSAI {
    board: BitBoard,
    me: i8,
    exploration: f64,
    //exploration: u16,
}

#[derive(Clone)]
struct TreeNode {
    pub board: BitBoard,
    //pub children: HashMap<u64, TreeNode>,
    pub children: Vec<TreeNode>,
    pub sim_count: u64,
    pub avg_reward: f64,
}

impl TreeNode {
    pub fn new(_board: BitBoard) -> TreeNode {
        TreeNode {
            board: _board,
            children: Vec::with_capacity(81),
            sim_count: 0,
            avg_reward: 0.0,
        }
    }
}

static rollouts_per_sim: u32 = 1;

impl AI for MCTSAI {
    fn get_move(&mut self, x_time: Duration, o_time: Duration) -> i64 {
        self.me = self.board.to_move;
        let mut time_remaining = Duration::from_millis(40000);
        let before = Instant::now();
        let mut tree = TreeNode::new(self.board.clone());
        let mut num_rollouts = 0;
        loop {
          for _i in 0..10 {
              self.rollout(&mut tree);
          }
          num_rollouts += 10 * rollouts_per_sim;
          let duration = Instant::now() - before;
          if duration > time_remaining {
              break;
          }
        }
        let mut best_count = 0;
        let mut best_move = 82;
        let mut best_reward = -1.0;
        let mut index = 0;
        BitBoard::iterate_moves(tree.board.get_moves(), &mut |m: u128, _sf: i64| {
            let node = &tree.children[index];
            let m = _sf;
            if node.sim_count > best_count {
                best_count = node.sim_count;
                best_move = m;
                best_reward = node.avg_reward;
            }
            index += 1;
            return true;
        });
        eprintln!("num_rollouts: {}", num_rollouts);
        eprintln!("best_reward: {}", best_reward);
        eprintln!("best_count: {}", best_count);
        self.board.make_move(1 << best_move);

        return best_move as i64;
    }

    fn make_move(&mut self, m: i64) {
        self.board.make_move(1 << m);
    }

    fn cleanup(&mut self) {

    }
}

impl MCTSAI {
    pub fn new(_exploration: f64) -> MCTSAI {
        MCTSAI {
            //exploration: ((1.0 - epsilon) * (std::u16::MAX as f64)) as u16,
            exploration: _exploration,
            board: BitBoard::new(),
            me: 1,
        }
    }

    pub fn rollout(&self, node: &mut TreeNode) -> f64 {
        let mut reward: f64 = 0.0;
        // Is the game ended?
        if node.board.get_winner() != 0 {
            node.sim_count += 1;
            if node.board.get_winner() == self.me {
                return rollouts_per_sim as f64;
            } else if node.board.get_winner() == -self.me {
                return 0.0;
            }
            return 0.0 * (rollouts_per_sim as f64);
        }
        // Is this a leaf?
        if node.children.len() == 0 {
            let mut moves = node.board.get_moves();
            // let board = &node.board;
            /*node.children.resize_with(moves.count_zeros() as usize, || {
              let m = (1 as u128) << (moves.trailing_zeros());
              moves &= !m;
              let mut new_board = board.clone();
              new_board.make_move(m);
              TreeNode::new(new_board)
            });*/
            while moves != 0 {
              let m = (1 as u128) << (moves.trailing_zeros());
              moves &= !m;
              let mut new_board = node.board.clone();
              new_board.make_move(m);
              //let mut new_node = TreeNode::new(new_board);
              node.children.push(TreeNode::new(new_board));
            }
            /*BitBoard::iterate_moves(node.board.get_moves(), &mut |m: u128, _sf: i64| {
              let mut new_board = node.board.clone();
              new_board.make_move(m);
              let mut new_node = TreeNode::new(new_board);
              node.children.push(new_node);
              return true;
            });*/
            let rand = (rand::random::<u8>() as usize) % node.children.len();
            let mut this_node = &mut node.children[rand];
            for _i in 0..rollouts_per_sim {
                reward += self.simulate(this_node.board.clone());
            }
            this_node.avg_reward = reward;
            this_node.sim_count = 1;
        } else {
            let mut move_score = -1000000000.0;
            let mut index = 0;
            for i in 0..node.children.len() {
                let n = &mut node.children[i];
                if (n.sim_count == 0) {
                    index = i;
                    break;
                }
                let score = ((node.board.to_move * self.me) as f64) *
                            (n.avg_reward) + self.exploration * ((node.sim_count as f64).log2()/(n.sim_count as f64)).sqrt();
                if score > move_score {
                    move_score = score;
                    index = i;
                }
            }
            //let mut new_node = node.children.get(&reward_move).unwrap().clone();
            let mut new_node = &mut node.children[index];
            reward = self.rollout(new_node);
        }
       /* let mut best_score = -1000000.0;
        node.sim_count += 1;
        for (m, n) in node.children.iter() {
            if n.avg_reward * (node.board.to_move as f64) * (self.me as f64) > best_score {
                best_score = n.avg_reward * (node.board.to_move as f64);
            }
        }
        node.avg_reward = best_score;*/
        node.sim_count += 1;
        node.avg_reward += (1.0 / (node.sim_count as f64)) * (reward - node.avg_reward);
        return reward;
    }
    
    pub fn simulate(&self, mut board: BitBoard) -> f64 {
        while board.get_winner() == 0 {
            board.make_move(BitBoard::random_move(board.get_moves()));
        }
        let winner = board.get_winner();
        if winner == self.me {
            return 1.0;
        } else if winner == -self.me {
            return 0.0;
        }
        return 0.5;
    }
}