mod basic {
    pub mod guessing_game;
    pub mod guessing_game_1;
}

use basic::guessing_game;
use basic::guessing_game_1;

fn main() {
    guessing_game::guessing_game_simple();
}

