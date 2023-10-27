mod basic {
    pub mod guessing_game;
    pub mod guessing_game_1;
    pub mod guessing_game_2;
    pub mod guessing_game_3;
    pub mod guessing_game_4;
    pub mod guessing_game_5;
}

mod variable {
    pub mod variable_immutable;
    pub mod variable_constants;
}

use basic::guessing_game;
use basic::guessing_game_1;
use basic::guessing_game_2;
use basic::guessing_game_3;
use basic::guessing_game_4;
use basic::guessing_game_5;
use variable::variable_immutable;
use variable::variable_constants;

fn main() {

    // Intro
    // guessing_game::guessing_game_simple();
    // guessing_game_1::variable_test();
    // guessing_game_1::placeholder_test();
    // guessing_game_2::guessing_game_by_random_number();
    // guessing_game_3::guessing_game_and_check();
    // guessing_game_4::guessing_game_until_correct();
    // guessing_game_5::guessing_game_compare_by_match();

    // variable
    // variable_immutable::immutable_test();
    variable_constants::constants_test();
}

