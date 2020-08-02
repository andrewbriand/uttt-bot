mod ai;
use ai::AI;

mod nn;

mod utttzero;
use utttzero::ZeroAI;

mod bitboard;
use bitboard::BitBoard;

use std::env;
use std::time::Duration;

struct TrainingRecord {
    board: BitBoard,
    result: f32,
    policy: Vec<f32>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ai1 = ZeroAI::from_file(1.2, args[1].clone());
    for i in 0..100 {
        let mut recs = Vec::new();
        ai1.reset();
        let mut board = BitBoard::new();
        while board.get_winner() == 0 {
            let (nm, policy) = ai1.get_move_example(Duration::from_millis(0), Duration::from_millis(0));
            recs.push(TrainingRecord { board: board.clone(), result: board.to_move as f32, policy: policy});
            board.make_move(1 << nm);
            if board.get_winner() != 0 {
                break;
            }
            let (nm, policy) = ai1.get_move_example(Duration::from_millis(0), Duration::from_millis(0));
            recs.push(TrainingRecord { board: board.clone(), result: board.to_move as f32, policy: policy});
            board.make_move(1 << nm);
        }
        for tr in recs {
            for i in 0..81 {
                if tr.board.x_occupancy & (1 << i) != 0 {
                    print!("1.0 ");
                } else if tr.board.o_occupancy & (1 << i) != 0 {
                    print!("-1.0 ");
                } else {
                    print!("0.0 ");
                }
            }
            println!();
            for p in tr.policy {
                print!("{} ", p);
            }
            println!("{}", tr.result * (board.get_winner() as f32));
        }
    }
}