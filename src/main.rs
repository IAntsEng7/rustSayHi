mod basic {
    pub mod guessing_game;
    pub mod guessing_game_1;
    pub mod guessing_game_2;
    pub mod guessing_game_3;
    pub mod guessing_game_4;
    pub mod guessing_game_5;
}

mod common_concepts {
    pub mod control_flow;
    pub mod data_type;
    pub mod function;
    pub mod variable;
}

mod practice {
    pub mod convert_temperatures;
    pub mod fibonacci_sequence;
}

use basic::guessing_game;
use basic::guessing_game_1;
use basic::guessing_game_2;
use basic::guessing_game_3;
use basic::guessing_game_4;
use basic::guessing_game_5;
use common_concepts::control_flow;
use common_concepts::data_type;
use common_concepts::function;
use common_concepts::variable;
use practice::convert_temperatures;
use practice::fibonacci_sequence;

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

    // data_type::compound_types();

    // function::function_test();

    // control_flow::ctrl_flow_if();
    // control_flow::ctrl_flow_if_else();
    // control_flow::ctrl_flow_if_else_if();
    // control_flow::ctrl_flow_loop();
    // control_flow::ctrl_flow_loop_break();
    // control_flow::ctrl_flow_while();
    // control_flow::ctrl_flow_for();

    // practice
    // practice::convert_temperatures
    // let fahrenheit_temperature = 98.6;
    // let celsius_temperature =
    //     convert_temperatures::convert_temperature(fahrenheit_temperature, "fahrenheit", "celsius");
    //
    // match celsius_temperature {
    //     Some(result) => {
    //         println!(
    //             "{} Fahrenheit is equal to {} Celsius",
    //             fahrenheit_temperature, result
    //         );
    //     }
    //     None => {
    //         println!("Invalid unit conversion");
    //     }
    // }
    // practice::fibonacci_sequence
    let n_th = 93;
    let result = fibonacci_sequence::fibonacci_at_n(n_th);
    println!("The {}th Fibonacci number is: {}", n_th, result)
}
