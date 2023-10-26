use rand::Rng;
use std::io;

fn create_random_num() -> u32 {
    let random_number = rand::thread_rng().gen_range(1..=100);
    random_number
}

pub fn guessing_game_and_check() {
    let random_num = create_random_num();

    println!("Please input your guess number");

    let mut guess_num = String::new();

    io::stdin().read_line(&mut guess_num)
        .expect("Failed to read line");

    // 將使用者輸入的字符串轉換為整數，並處理可能的錯誤
    let guess_num: u32 = match guess_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    println!("=========================================================");

    // 檢查使用者猜測的數字是否與隨機生成的數字相同
    if guess_num == random_num {
        println!("Congratulations! You guessed the correct number.");
    } else {
        println!("Sorry, you didn't guess the correct number.");
        println!("You guessed: {}", guess_num);
    }

    println!("Secret number: {random_num}");
}