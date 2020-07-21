
use crate::ai::AI;
use crate::bitboard::BitBoard;

use std::time::{Duration, Instant};
use std::collections::HashMap;

use rand;

pub struct MCTSAI {
    board: BitBoard,
    me: i8,
    exploration: u16,
}

#[derive(Clone)]
struct TreeNode {
    pub board: BitBoard,
    pub children: HashMap<u64, TreeNode>,
    pub sim_count: u64,
    pub avg_reward: f64,
}

impl TreeNode {
    pub fn new(_board: BitBoard) -> TreeNode {
        TreeNode {
            board: _board,
            children: HashMap::new(),
            sim_count: 0,
            avg_reward: 0.0,
        }
    }
}

static rollouts_per_sim: u32 = 10;

impl AI for MCTSAI {
    fn get_move(&mut self, x_time: Duration, o_time: Duration) -> i64 {
        self.me = self.board.to_move;
        let mut time_remaining = Duration::from_millis(90);
        let mut tree = TreeNode::new(self.board.clone());
        let mut num_rollouts = 0;
        loop {
          let before = Instant::now();
          for _i in 0..10 {
              self.rollout(&mut tree);
          }
          num_rollouts += 10 * rollouts_per_sim;
          let duration = Instant::now() - before;
          if time_remaining < duration {
              break;
          }
          time_remaining -= duration;
        }
        let mut best_count = 0;
        let mut best_move = 82;
        let mut best_reward = -1.0;
        for (m, node) in tree.children {
            if node.sim_count > best_count {
                best_count = node.sim_count;
                best_move = m;
                best_reward = node.avg_reward;
            }
        }
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
    pub fn new(epsilon: f64) -> MCTSAI {
        MCTSAI {
            exploration: ((1.0 - epsilon) * (std::u16::MAX as f64)) as u16,
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
            return 0.5 * (rollouts_per_sim as f64);
        }
        // Is this a leaf?
        if node.children.len() == 0 {
            let rand_move = BitBoard::random_move(node.board.get_moves()).trailing_zeros() as u64;
            let mut new_board = node.board.clone();
            new_board.make_move(1 << rand_move);
            for _i in 0..rollouts_per_sim {
                reward += self.simulate(new_board.clone());
            }
            let mut new_node = TreeNode::new(new_board);
            new_node.avg_reward = reward;
            new_node.sim_count = 1;
            node.children.insert(rand_move, new_node);
        } else {
            if rand::random::<u16>() < self.exploration {
                let rand_move = BitBoard::random_move(node.board.get_moves()).trailing_zeros() as u64;
                if !node.children.contains_key(&rand_move) {
                    let mut new_board = node.board.clone();
                    new_board.make_move(1 << rand_move);
                    node.children.insert(rand_move, TreeNode::new(new_board));
                }
                //let mut new_node = node.children.get(&rand_move).unwrap().clone();
                //reward = self.rollout(&mut new_node);
                //node.children.insert(rand_move, new_node);
                let mut new_node = node.children.get_mut(&rand_move).unwrap();
                reward = self.rollout(new_node);
            } else {
                let mut reward = -1000000000.0;
                let mut reward_move = 82;
                for (m, n) in node.children.iter() {
                    if ((node.board.to_move * self.me) as f64) * n.avg_reward > reward {
                        reward = n.avg_reward;
                        reward_move = *m;
                    }
                }
                //let mut new_node = node.children.get(&reward_move).unwrap().clone();
                let mut new_node = node.children.get_mut(&reward_move).unwrap();
                reward = self.rollout(new_node);
                //node.children.insert(reward_move, new_node);
            }

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
    
    pub fn simulate(&self, mut board: BitBoard) -> f64{
        while board.get_winner() == 0 {
            board.make_move(BitBoard::random_move(board.get_moves()));
        }
        if board.get_winner() == self.me {
            return 1.0;
        } else if board.get_winner() == -self.me {
            return 0.0;
        }
        return 0.5;
    }
}