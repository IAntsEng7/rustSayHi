use std::io;

pub fn guessing_game_simple() {
    println!("Please input your guess number");

    let mut guess_num = String::new();

    io::stdin().read_line(&mut guess_num)
        .expect("Failed to read line");

    println!("You guessed: {}", guess_num);
}