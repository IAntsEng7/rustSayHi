mod basic {
    pub mod guessing_game;
    pub mod guessing_game_1;
    pub mod guessing_game_2;
    pub mod guessing_game_3;
    pub mod guessing_game_4;
    pub mod guessing_game_5;
}

mod common_concepts {
    pub mod variable;
    pub mod data_type;
}

use basic::guessing_game;
use basic::guessing_game_1;
use basic::guessing_game_2;
use basic::guessing_game_3;
use basic::guessing_game_4;
use basic::guessing_game_5;
use common_concepts::variable;
use common_concepts::data_type;

fn main() {

    // Intro
    // guessing_game::guessing_game_simple();
    // guessing_game_1::variable_test();
    // guessing_game_1::placeholder_test();
    // guessing_game_2::guessing_game_by_random_number();
    // guessing_game_3::guessing_game_and_check();
    // guessing_game_4::guessing_game_until_correct();
    // guessing_game_5::guessing_game_compare_by_match();

    // common_concepts
    // variable::immutable_test();
    // variable::constants_test();
    // variable::shadowing_test_1();
    // variable::shadowing_test_2();
    data_type::compound_types();
}

