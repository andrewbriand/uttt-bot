mod ai;
use ai::AI;

//mod simplesearch;
//use simplesearch::SimpleSearchAI;

mod mcts;
use mcts::MCTSAI;

use std::io;

mod bitboard;

//use text_io::read;

use std::time::Duration;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


fn main() {
    //let mut curr_ai : Box<dyn AI> = Box::new(SimpleSearchAI::new(SimpleSearchAI::abriand_eval_1(), 8));
    let mut curr_ai : Box<dyn AI> = Box::new(MCTSAI::new(1.35));


	let mut rollouts = 0;
        let num_trials = 1;
        for _i in 0..num_trials {
          let (best_move, r) = curr_ai.get_move_rollouts(Duration::from_millis(0), Duration::from_millis(0));
          rollouts += r;
        }
       // eprintln!("best_move: {}", best_move);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", rollouts/num_trials);
}
