use rand::Rng;
use std::io;

fn create_random_num() -> u32 {
    let random_number = rand::thread_rng().gen_range(1..=100);
    random_number
}

pub fn guessing_game_by_random_number() {
    let random_num = create_random_num();

    println!("Please input your guess number");

    let mut guess_num = String::new();

    io::stdin()
        .read_line(&mut guess_num)
        .expect("Failed to read line");

    println!("You guessed: {}", guess_num);
    println!("Secret number: {random_num}");
}
