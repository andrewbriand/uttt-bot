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
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_row = parse_input!(inputs[0], i64);
        let opponent_col = parse_input!(inputs[1], i64);
        if (opponent_row != -1 && opponent_col != -1) {
            curr_ai.make_move((opponent_col/3)*9 + (opponent_col % 3) 
                               + (opponent_row/3)*27 + 3*(opponent_row % 3));
//            board.make_move((1 as u16) << (opponent_row*3 + opponent_col));
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let valid_action_count = parse_input!(input_line, i32);
        for i in 0..valid_action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
           // let inputs = input_line.split(" ").collect::<Vec<_>>();
           // let row = parse_input!(inputs[0], i32);
           // let col = parse_input!(inputs[1], i32);
            

        }


        let best_move = curr_ai.get_move(Duration::from_millis(0), Duration::from_millis(0));
       // eprintln!("best_move: {}", best_move);
        let response_row = ((best_move/9) % 3)*3 + best_move % 3;
        let response_col = ((best_move/9) / 3)*3 + (best_move % 9)/3;
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{} {}", response_col, response_row);
    }
}
