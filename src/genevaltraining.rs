mod oldmcts;
use oldmcts::MCTSAI;

mod bitboard;
use bitboard::BitBoard;

use std::time::Duration;

fn main() {
  for i in 0..10 {
  let mut board = BitBoard::new();

  let mut ai1 = MCTSAI::new(1.0);

  let mut ai2 = MCTSAI::new(1.0);

  while board.get_winner() == 0 {
    let (m, eval) = ai1.get_move(Duration::from_millis(5000));
    for i in 0..90 {
	if board.x_occupancy & (1 << i) != 0 {
	    print!("{} ", 1.0 * (board.to_move as f64));
	} else if board.o_occupancy & (1 << i) != 0 {
	    print!("{} ", -1.0 * (board.to_move as f64));
	} else {
	    print!("0.0 ");
	}
    }

    println!("{}", eval);

    for _i in 0..90 {
        let i;
        if _i <= 80 {
          i = 80 - _i;
        } else {
          i = 81 + (8 - (_i - 81));
        }
	if board.x_occupancy & (1 << i) != 0 {
	    print!("{} ", 1.0 * (board.to_move as f64));
	} else if board.o_occupancy & (1 << i) != 0 {
	    print!("{} ", -1.0 * (board.to_move as f64));
	} else {
	    print!("0.0 ");
	}
    }

    println!("{}", eval);
    
    board.make_move(1 << m);
    ai2.make_move(m);
    if board.get_winner() != 0 {
      break;
    }
    let (m, eval) = ai2.get_move(Duration::from_millis(5000));
    for i in 0..90 {
	if board.x_occupancy & (1 << i) != 0 {
	    print!("{} ", 1.0 * (board.to_move as f64));
	} else if board.o_occupancy & (1 << i) != 0 {
	    print!("{} ", -1.0 * (board.to_move as f64));
	} else {
	    print!("0.0 ");
	}
    }

    println!("{}", eval);

    for _i in 0..90 {
        let i;
        if _i <= 80 {
          i = 80 - _i;
        } else {
          i = 81 + (8 - (_i - 81));
        }
	if board.x_occupancy & (1 << i) != 0 {
	    print!("{} ", 1.0 * (board.to_move as f64));
	} else if board.o_occupancy & (1 << i) != 0 {
	    print!("{} ", -1.0 * (board.to_move as f64));
	} else {
	    print!("0.0 ");
	}
    }

    println!("{}", eval);
    
    board.make_move(1 << m);
    ai1.make_move(m);
  }
  }
}
