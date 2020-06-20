use crate::ai::AI;
use crate::bitboard::BitBoard;

pub struct KillerMoveAI {
    board: BitBoard,
    eval: Box<dyn Fn(&mut BitBoard, i8) -> i32>,
    depth: usize,
    me: i8,
    killers: Vec<Vec<u128>>,
    killer_hits: u64,
    moves_examined: u64,
}

impl AI for KillerMoveAI {

    fn get_move(&mut self, last_move: i64) -> i64 {
        if last_move != -1 {
            self.board.make_move(1 << last_move);
        }
        self.me = self.board.to_move;
        let alpha = -100000000;
        let beta = 100000000;
        let (result_move, result_score) = self.search(&mut self.board.clone(), self.depth, alpha, beta);
        println!("killer hits: {}/{}", self.killer_hits, self.moves_examined);
        println!("result score: {}", result_score);
        self.board.make_move(1 << result_move);
        return result_move;
    }

    fn cleanup(&mut self) {}
}

impl KillerMoveAI {
    pub fn new<'a>(_eval: Box<dyn Fn(&mut BitBoard, i8) -> i32>, _depth: usize) 
        -> KillerMoveAI {
        KillerMoveAI {
            board: BitBoard::new(),
            eval: _eval,
            depth: _depth,
            me: 0,
            killers: vec![vec![0; 3]; _depth],
            killer_hits: 0,
            moves_examined: 0,
        }
    }

    fn insert_to_killers(killers: &mut Vec<u128>, m: u128) {
        for i in 0..killers.len() {
            if killers[i] == m {
                return;
            }
            if killers[i] == 0 {
                killers[i] = m;
                return;
            }
        }
        killers[0] = m;
    }

    pub fn search(&mut self, board: &mut BitBoard, depth: usize, 
                  _alpha: i32, beta: i32) -> (i64, i32) {
        let mut alpha = _alpha;
        if depth == 0 {
            return (-1, (self.eval)(board, self.me));
        }
        let moves = board.get_moves();
        if moves == 0 {
            if depth % 2 == 0 {
                return (-1, (self.eval)(board, self.me));
            } else {
                return (-1, -(self.eval)(board, self.me));
            }
        }
        let mut result_move = -1;
        for i in 0..3 {
            let killer = self.killers[self.depth - depth][i];
            if killer != 0 {
                if moves & killer != 0 {
                    let mut next_b = board.clone();
                    next_b.make_move(killer);
                    let (_, mut score) = self.search(&mut next_b, depth - 1, -beta, -alpha);
                    score = -score;
                    if score > alpha {
                        alpha = score;
                        result_move = BitBoard::mask_to_sf(killer) as i64;
                    }

                    self.moves_examined += 1;

                    if alpha >= beta {
                        //println!("killer hit!");
                        self.killer_hits += 1;
                        return (result_move, alpha);
                    }
                }
            }
        }
        BitBoard::iterate_moves(moves, &mut |next_move: u128, next_move_sf: i64| {
           let mut next_b = board.clone();
           next_b.make_move(next_move);
           let (_, mut score) = self.search(&mut next_b, depth - 1, -beta, -alpha);
           score = -score;
           if score > alpha {
               alpha = score;
               result_move = next_move_sf;
           }
           self.moves_examined += 1;

           if alpha >= beta {
               KillerMoveAI::insert_to_killers(&mut self.killers[self.depth - depth], next_move);
               return false;
           }
           return true;
        });
        return (result_move, alpha);
    }

    /*pub fn ab_then_mc(games: usize) -> Box<dyn Fn(&mut BitBoard, Player) -> i32> {
        Box::new(move |_board: &mut BitBoard, me: Player| -> i32 {
              let opponent = match me {
                  Player::X => Player::O,
                  Player::O => Player::X,
                  _ => panic!("AI is not a player"),
              };
              if _board.winner == me {
                 return 50000;
              } else if _board.winner == opponent {
                 return -50000;
              }
            let mut result = 0;
            for _i in 0..games {
                let mut board = _board.clone();
                while board.winner == Player::NEITHER {
                    let moves = board.get_moves();
                    let next_move = moves[rand::random::<usize>() % moves.len()];
                    board.make_move(next_move);
                }
                if board.winner == me {
                    result += 1;
                } else if board.winner == opponent {
                    result += -1;
                }
            }
            return result;
        })
    }*/

    pub fn abriand_eval_1() -> Box<dyn Fn(&mut BitBoard, i8) -> i32> {
        Box::new(move |board: &mut BitBoard, me: i8| -> i32 {
              if board.get_winner() == me {
                 return 50000;
              } else if board.get_winner() == -me {
                 return -50000;
              }
              let mut result : i32 = 0;
              for i in 0..9 {
                  if board.x_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result += (me as i32) * 1000;
                  } else if board.o_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result -= (me as i32) * 1000;
                  }
              }
              if board.x_occupancy & ((1 as u128) << (81 + 4)) != 0 {
                  result += (me as i32) * 1000;
              } else if board.o_occupancy & ((1 as u128) << (81 + 4)) != 0 {
                  result -= (me as i32) * 1000;
              }
              
              for i in [4, 13, 22, 31, 40, 49, 58, 67, 76].iter() {
                  if board.x_occupancy & ((1 as u128) << i) != 0 as u128 {
                      result += (me as i32) * 100;
                  } else if board.o_occupancy & ((1 as u128) << i) != 0 as u128 {
                      //result -= (me as i32) * 100;
                  }
              }
              return result;
        })
    }
    /*pub fn diagonal() -> Box<dyn Fn(&mut Board, Player) -> i32> {
        Box::new(move |board: &mut Board, me: Player| -> i32 {
              let opponent = match me {
                  Player::X => Player::O,
                  Player::O => Player::X,
                  _ => panic!("AI is not a player"),
              };
              if board.winner == me {
                 return 50000;
              } else if board.winner == opponent {
                 return -50000;
              }
              let mut result = 0;
                  match board.get(Square { top_left: 36,
                                        level: 1}) {
                        x if me == x => result += 1000,
                        x if opponent == x => result -= 1000,
                        _ => ()
                   }
              for i in [0, 36, 72].iter() {
                  match board.get(Square { top_left: *i,
                                        level: 1}) {
                        x if me == x => result += 1000,
                        x if opponent == x => result -= 1000,
                        _ => ()
                   }
              }
              return result;
        })
    }*/
}