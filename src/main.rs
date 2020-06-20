mod ai;
use ai::AI;

mod simplesearch;
use simplesearch::SimpleSearchAI;

mod bitboard;

use text_io::read;


fn main() {
    let mut curr_ai : Box<dyn AI> = 
        Box::new(SimpleSearchAI::new(SimpleSearchAI::diagonal2(), 10));
    loop {
        println!("{}", curr_ai.get_move(read!()));
    }
}
