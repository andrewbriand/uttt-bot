use crate::ai::AI;
use crate::bitboard::BitBoard;

pub struct ShortExtendAI {
    board: BitBoard,
    eval: Box<dyn Fn(&mut BitBoard, i8) -> i32>,
    depth: usize,
    me: i8,
}

impl AI for ShortExtendAI {

    fn get_move(&mut self, last_move: i64) -> i64 {
        if last_move != -1 {
            self.board.make_move(1 << last_move);
        }
        self.me = self.board.to_move;
        let alpha = -100000000;
        let beta = 100000000;
        let (result_move, result_score) = self.search(&mut self.board.clone(), self.depth, alpha, beta, 0);
        println!("result score: {}", result_score);
        self.board.make_move(1 << result_move);
        return result_move;
    }

    fn cleanup(&mut self) {}
}

impl ShortExtendAI {
    pub fn new<'a>(_eval: Box<dyn Fn(&mut BitBoard, i8) -> i32>, _depth: usize) 
        -> ShortExtendAI {
        ShortExtendAI {
            board: BitBoard::new(),
            eval: _eval,
            depth: _depth,
            me: 0,
        }
    }

    pub fn search(&self, board: &mut BitBoard, depth: usize, 
                  _alpha: i32, beta: i32, depth_so_far: usize) -> (i64, i32) {
        let mut alpha = _alpha;
        if depth == 0 {
            let eval = (self.eval)(board, self.me);
            if depth_so_far < 15 && eval > 1000 {
                return self.search(board, 4, _alpha, beta, depth_so_far);
            }
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
        BitBoard::iterate_moves(moves, &mut |next_move: u128, next_move_sf: i64| {
           let mut next_b = board.clone();
           next_b.make_move(next_move);
           let (_, mut score) = self.search(&mut next_b, depth - 1, -beta, -alpha, depth_so_far + 1);
           score = -score;
           if score > alpha {
               alpha = score;
               result_move = next_move_sf;
           }

           if alpha >= beta {
               return false;
           }
           return true;
        });
        return (result_move, alpha);
    }

    fn branching_mc(board: &mut BitBoard, branching: u8, me: i8) -> i32 {
        if board.get_winner() == me {
            return 1;
        } else if board.get_winner() == -me {
            return -1;
        } else if board.get_winner() == -2 {
            return 0;
        }
        let mut result = 0;
        BitBoard::iterate_moves(board.get_moves(), &mut |m: u128, _sf: i64| {
            if rand::random::<u8>() < branching {
                let mut n_b = board.clone();
                n_b.make_move(m);
                result += ShortExtendAI::branching_mc(&mut n_b, branching, me);
            }
            return true;
        });
        return result;
    }

    pub fn ab_then_mc(games: u64) -> Box<dyn Fn(&mut BitBoard, i8) -> i32> {
        Box::new(move |_board: &mut BitBoard, me: i8| -> i32 {
              if _board.get_winner() == me {
                 return 50000;
              } else if _board.get_winner() == -me {
                 return -50000;
              } else if _board.get_winner() == -2 {
                  return 0;
              }
              let mut result = 0;
              for _i in 0..games {
                  let mut new_board = _board.clone();
                  while new_board.get_winner() == 0 {
                      new_board.make_move(BitBoard::random_move(new_board.get_moves()));
                  } 
                  if new_board.get_winner() == me {
                      result += 1;
                  } else if new_board.get_winner() == -me {
                      result -= 1;
                  }
              }
              return result;
        })
    }

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
                      result -= (me as i32) * 100;
                  }
              }
              return result;
        })
    }

    fn num_occupied_x(board: &mut BitBoard, cells: Vec<i32>) -> u32 {
        let mut r = 0;
        for i in cells.iter() {
            if board.x_occupancy & ((1 as u128) << i) != 0 {
                r += 1;
            }
        }
        return r;
    }

    fn num_occupied_o(board: &mut BitBoard, cells: Vec<i32>) -> u32 {
        let mut r = 0;
        for i in cells.iter() {
            if board.o_occupancy & ((1 as u128) << i) != 0 {
                r += 1;
            }
        }
        return r;
    }

    pub fn diagonal() -> Box<dyn Fn(&mut BitBoard, i8) -> i32> {
        Box::new(move |board: &mut BitBoard, me: i8| -> i32 {
              if board.get_winner() == me {
                 return 50000;
              } else if board.get_winner() == -me {
                 return -50000;
              }
              let mut partial_credit;
              let mut result : i32 = 0;
              for i in 0..9 {
                  if i == 4 {
                      partial_credit = 800;
                  } else {
                      partial_credit = 400;
                  }
                  if board.x_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result += (me as i32) * 1000;
                  } else if board.o_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result -= (me as i32) * 1000;
                  } else {
                      for j in 0..3 {
                        if ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 0 {
                          result += (me as i32) * partial_credit;
                        } 
                        if ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 0 {
                          result -= (me as i32) * partial_credit;
                        }
                      }
                      for j in [0, 3, 6].iter() {
                        if ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 0 {
                          result += (me as i32) * partial_credit;
                        }
                        if ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 0 {
                          result -= (me as i32) * partial_credit;
                        }
                      }
                      if ShortExtendAI::num_occupied_x(board, vec![9*i, 9*i + 4, 9*i + 8]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i, 9*i + 4, 9*i + 8]) == 0 {
                        result += (me as i32) * partial_credit;
                      } 
                      if ShortExtendAI::num_occupied_o(board, vec![9*i, 9*i + 4, 9*i + 8]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i, 9*i + 4, 9*i + 8]) == 0 {
                        result -= (me as i32) * partial_credit;
                      }
                      if ShortExtendAI::num_occupied_x(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 0 {
                        result += (me as i32) * partial_credit;
                      }
                      if ShortExtendAI::num_occupied_o(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 0 {
                        result -= (me as i32) * partial_credit;
                      }
                  }
              }
              let partial_credit_l2 = 6000;
              for j in 0..3 {
                if ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 0 {
                  result += (me as i32) * partial_credit_l2;
                } 
                if ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 0 {
                  result -= (me as i32) * partial_credit_l2;
                }
              }
              for j in [0, 3, 6].iter() {
                if ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 0 {
                  result += (me as i32) * partial_credit_l2;
                }
                if ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 0 {
                  result -= (me as i32) * partial_credit_l2;
                }
              }
              if ShortExtendAI::num_occupied_x(board, vec![81, 81 + 4, 81 + 8]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81, 81 + 4, 81 + 8]) == 0 {
                result += (me as i32) * partial_credit_l2;
              } 
              if ShortExtendAI::num_occupied_o(board, vec![81, 81 + 4, 81 + 8]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81, 81 + 4, 81 + 8]) == 0 {
                result -= (me as i32) * partial_credit_l2;
              }
              if ShortExtendAI::num_occupied_x(board, vec![81 + 2, 81 + 4, 81 + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + 2, 81 + 4, 81 + 6]) == 0 {
                result += (me as i32) * partial_credit_l2;
              }
              if ShortExtendAI::num_occupied_o(board, vec![81 + 2, 81 + 4, 81 + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + 2, 81 + 4, 81 + 6]) == 0 {
                result -= (me as i32) * partial_credit_l2;
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
                      result -= (me as i32) * 100;
                  }
              }
              
              
              for i in 0..9 {
                  for j in [0, 2, 4, 6, 8].iter() {
                    if board.x_occupancy & ((1 as u128) << (9 * i + j)) != 0 as u128 {
                        result += (me as i32) * 100;
                    } else if board.o_occupancy & ((1 as u128) << (9 * i + j)) != 0 as u128 {
                        result -= (me as i32) * 100;
                    }
                  }
              }
                  /*for j in [0, 2, 4, 6, 8].iter() {
                    if board.x_occupancy & ((1 as u128) << (81 + j)) != 0 as u128 {
                        result += (me as i32) * 500;
                    } else if board.o_occupancy & ((1 as u128) << (81 + j)) != 0 as u128 {
                        result -= (me as i32) * 500;
                    }
                  }*/
              return result + ((rand::random::<i32>() % 200) - 100);
        })
    }

    pub fn centers() -> Box<dyn Fn(&mut BitBoard, i8) -> i32> {
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
              for i in [4, 13, 22, 31, 40, 49, 58, 67, 76].iter() {
                  if board.x_occupancy & ((1 as u128) << i) != 0 as u128 {
                      result += (me as i32) * 100;
                  } else if board.o_occupancy & ((1 as u128) << i) != 0 as u128 {
                      result -= (me as i32) * 100;
                  }
              }
              return result + ((rand::random::<i32>() % 100) - 50);
        })
    }

    pub fn diagonal2() -> Box<dyn Fn(&mut BitBoard, i8) -> i32> {
        Box::new(move |board: &mut BitBoard, me: i8| -> i32 {
              if board.get_winner() == me {
                 return 50000;
              } else if board.get_winner() == -me {
                 return -50000;
              }
              let mut partial_credit;
              let mut result : i32 = 0;
              for i in 0..9 {
                  if i == 4 {
                      partial_credit = 600;
                  } else {
                      partial_credit = 400;
                  }
                  if board.x_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result += (me as i32) * 1000;
                  } else if board.o_occupancy & ((1 as u128) << (81 + i)) != 0 {
                      result -= (me as i32) * 1000;
                  } else {
                      for j in 0..3 {
                        if ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 0 {
                          result += (me as i32) * partial_credit;
                        } 
                        if ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 3, 9*i + j + 6]) == 0 {
                          result -= (me as i32) * partial_credit;
                        }
                      }
                      for j in [0, 3, 6].iter() {
                        if ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 0 {
                          result += (me as i32) * partial_credit;
                        }
                        if ShortExtendAI::num_occupied_o(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + j, 9*i + j + 1, 9*i + j + 2]) == 0 {
                          result -= (me as i32) * partial_credit;
                        }
                      }
                      if ShortExtendAI::num_occupied_x(board, vec![9*i, 9*i + 4, 9*i + 8]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i, 9*i + 4, 9*i + 8]) == 0 {
                        result += (me as i32) * partial_credit;
                      } 
                      if ShortExtendAI::num_occupied_o(board, vec![9*i, 9*i + 4, 9*i + 8]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i, 9*i + 4, 9*i + 8]) == 0 {
                        result -= (me as i32) * partial_credit;
                      }
                      if ShortExtendAI::num_occupied_x(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 0 {
                        result += (me as i32) * partial_credit;
                      }
                      if ShortExtendAI::num_occupied_o(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![9*i + 2, 9*i + 4, 9*i + 6]) == 0 {
                        result -= (me as i32) * partial_credit;
                      }
                  }
              }
              let partial_credit_l2 = 6000;
              for j in 0..3 {
                if ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 0 {
                  result += (me as i32) * partial_credit_l2;
                } 
                if ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 3, 81 + j + 6]) == 0 {
                  result -= (me as i32) * partial_credit_l2;
                }
              }
              for j in [0, 3, 6].iter() {
                if ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 0 {
                  result += (me as i32) * partial_credit_l2;
                }
                if ShortExtendAI::num_occupied_o(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + j, 81 + j + 1, 81 + j + 2]) == 0 {
                  result -= (me as i32) * partial_credit_l2;
                }
              }
              if ShortExtendAI::num_occupied_x(board, vec![81, 81 + 4, 81 + 8]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81, 81 + 4, 81 + 8]) == 0 {
                result += (me as i32) * partial_credit_l2;
              } 
              if ShortExtendAI::num_occupied_o(board, vec![81, 81 + 4, 81 + 8]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81, 81 + 4, 81 + 8]) == 0 {
                result -= (me as i32) * partial_credit_l2;
              }
              if ShortExtendAI::num_occupied_x(board, vec![81 + 2, 81 + 4, 81 + 6]) == 2 
                           && ShortExtendAI::num_occupied_o(board, vec![81 + 2, 81 + 4, 81 + 6]) == 0 {
                result += (me as i32) * partial_credit_l2;
              }
              if ShortExtendAI::num_occupied_o(board, vec![81 + 2, 81 + 4, 81 + 6]) == 2 
                           && ShortExtendAI::num_occupied_x(board, vec![81 + 2, 81 + 4, 81 + 6]) == 0 {
                result -= (me as i32) * partial_credit_l2;
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
                      result -= (me as i32) * 100;
                  }
              }
              
              
              for i in 0..9 {
                  for j in [0, 2, 4, 6, 8].iter() {
                    if board.x_occupancy & ((1 as u128) << (9 * i + j)) != 0 as u128 {
                        result += (me as i32) * 100;
                    } else if board.o_occupancy & ((1 as u128) << (9 * i + j)) != 0 as u128 {
                        result -= (me as i32) * 100;
                    }
                  }
              }
                  /*for j in [0, 2, 4, 6, 8].iter() {
                    if board.x_occupancy & ((1 as u128) << (81 + j)) != 0 as u128 {
                        result += (me as i32) * 500;
                    } else if board.o_occupancy & ((1 as u128) << (81 + j)) != 0 as u128 {
                        result -= (me as i32) * 500;
                    }
                  }*/
              return result + ((rand::random::<i32>() % 200) - 100);
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