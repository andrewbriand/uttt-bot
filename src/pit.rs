mod ai;
use ai::AI;

mod nn;

mod utttzero;
use utttzero::ZeroAI;

mod bitboard;
use bitboard::BitBoard;

use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ai1 = ZeroAI::from_file(1.2, args[1].clone());
    let mut ai2 = ZeroAI::from_file(1.2, args[2].clone());

    let mut win_count1 = 0;
    let mut win_count2 = 0;
    for i in 0..100 {
        let mut board = BitBoard::new();
        ai1.reset();
        ai2.reset();
        if i % 2 == 0 {
            let nm = ai2.get_move(Duration::from_millis(0), Duration::from_millis(0));
            ai1.make_move(nm);
            board.make_move(1 << nm);
        }
        loop {
            let nm = ai1.get_move(Duration::from_millis(0), Duration::from_millis(0));
            ai2.make_move(nm);
            board.make_move(1 << nm);
            if board.get_winner() != 0 {
                break;
            }
            let nm = ai2.get_move(Duration::from_millis(0), Duration::from_millis(0));
            ai1.make_move(nm);
            board.make_move(1 << nm);
            if board.get_winner() != 0 {
                break;
            }
        }
        if board.get_winner() == 1 {
            if i % 2 == 0 { // ai2 is X
                win_count2 += 1;
            } else {
                win_count1 += 1;
            }
        } else if board.get_winner() == -1 {
            if i % 2 == 1 { // ai2 is O
                win_count2 += 1;
            } else {
                win_count1 += 1;
            }
        }
    }
    println!("{} {}", win_count1, win_count2);
}