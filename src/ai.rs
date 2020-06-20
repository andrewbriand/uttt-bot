pub trait AI {
    // returns the move the AI wants to make
    // given that the last move was last_move
    // last_move should be -1 if this is the first move of the game
    // get_move returns -1 if the ai wants to resign
    fn get_move(&mut self, last_move : i64) -> i64;

    fn cleanup(&mut self);
}