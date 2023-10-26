use std::io;

fn main() {
    // println!("Please input your guess number");
    // let mut guess_num = String::new(); // String guessNum = new String();
    // io::stdin().read_line(&mut guess_num)
    //     .expect("Failed to read line");
    //
    // println!("You guessed: {}", guess_num);
    variable_test();
    placeholder_test();
}

// 1. Variable: mutable(可變)and immutable(不可變)
fn variable_test() {
    let var1 = "It can NOT change.";
    let mut var2 = "It can change.";
    println!("var1: {}", var1);
    println!("var2: {}", var2);

    // var1 = "change failed";
    // Cannot assign a new value to an immutable variable more than once [E0384]
    var2 = "I change it!!";
    println!("update var2: {}", var2);

    // Output:
    // var1: It can NOT change.
    // var2: It can change.
    // update var2: I change it!!
}

// 2. placeholder
fn placeholder_test() {
    let placeholder1 = "RustRover";
    let placeholder2 = "IDE";
    // single
    println!("I use {} as IDE to write Rust.", placeholder1);
    // multiple
    println!("{} is a {} of Rust Language.", placeholder1, placeholder2);

    // Output:
    // I use RustRover as IDE to write Rust.
    // RustRover is a IDE of Rust Language.
}