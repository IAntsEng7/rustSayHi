mod basic {
    pub mod guessing_game;
    pub mod guessing_game_1;
    pub mod guessing_game_2;
}

use basic::guessing_game;
use basic::guessing_game_1;
use basic::guessing_game_2;

fn main() {
    // guessing_game::guessing_game_simple();
    // guessing_game_1::variable_test();
    // guessing_game_1::placeholder_test();
    guessing_game_2::guessing_game_by_random_number();
}

