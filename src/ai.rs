
use std::time::Duration;

pub trait AI {
    // returns the move the AI wants to make
    // and makes the move on its internal board representation
    // given the current board state based on previous
    // calls to make_move
    fn get_move(&mut self, x_time: Duration, o_time: Duration) -> i64;

    // returns the move the AI wants to make
    // and makes the move on its internal board representation
    // given the current board state based on previous
    // calls to make_move
    fn get_move_rollouts(&mut self, x_time: Duration, o_time: Duration) -> (i64, u32);

    // Tell the ai to make the passed move on its internal 
    // board representation
    fn make_move(&mut self, m: i64);

    fn cleanup(&mut self);
}
