mod ai;
use ai::AI;

mod simplesearch;
use simplesearch::SimpleSearchAI;

mod bitboard;

use text_io::read;


fn main() {
    let mut curr_ai : Box<dyn AI> = 
        Box::new(SimpleSearchAI::new(SimpleSearchAI::diagonal2(), 10));
    let next: String = read!();
    if next == "uti" {
        println!("utiok");
    } else {
        return;
    }
    loop {
        let mut m: i64;
        let cmd: String = read!();
        let mut subcmd: String;
        if cmd == "pos".to_string() {
            subcmd = read!();
            if subcmd == "moves".to_string() {
              curr_ai.make_move(read!());
            }
        } else if cmd == "search".to_string() {
            subcmd = read!();
            if subcmd == "free".to_string() {
                println!("info best_move={}", curr_ai.get_move());
            }
        }
    }
}
