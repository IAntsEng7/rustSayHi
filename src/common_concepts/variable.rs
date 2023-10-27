pub fn immutable_test() {
    let var_1 = "Only can assign once!!";
    let mut var_2 = "Try to assign multiple times";

    println!("1. var_1: {var_1}");
    println!("1. var_2: {var_2}");

    // var_1 = "try";
    println!("2. var_1: Cannot assign a new value to an immutable variable more than once.");
    var_2 = "Assign second times.";
    println!("2. var_2: {var_2}")

    // Output:
    // 1. var_1: Only can assign once!!
    // 1. var_2: Try to assign multiple times
    // 2. var_1: Cannot assign a new value to an immutable variable more than once.
    // 2. var_2: Assign second times.
}

pub fn constants_test() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("It's {THREE_HOURS_IN_SECONDS} seconds.");

    // THREE_HOURS_IN_SECONDS = 1 * 2 * 3;
    println!("Can I assign THREE_HOURS_IN_SECONDS multiple times?")

    // Output:
    // 5 |     THREE_HOURS_IN_SECONDS = 1 * 2 * 3;
    //   |     ---------------------- ^
    //   |     |
    //   |     cannot assign to this expression
}

pub fn shadowing_test_1() {
    let x = 7;
    println!("let x = 7: {x}");

    let x = x + 1;
    println!("let x = x + 1: {x}");

    {
        let x = x * 2;
        println!("let x = x * 2: {x}");
    }

    println!("Last x: {x}");

    // Output:
    // let x = 7: 7
    // let x = x + 1: 8
    // let x = x * 2: 16
    // Last x: 8
}

pub fn shadowing_test_2(){
    let spaces = "     ";
    println!("1. What is <{spaces}>");
    let spaces = spaces.len();
    println!("2. What is <{spaces}>");

    // Output:
    // 1. What is <     >
    // 2. What is <5>

    let mut spaces_2 = "   ";
    // spaces_2 = spaces.len();
    println!("3. What spaces_2 is? <{spaces}>");
    // Output:
    // error[E0599]: no method named `len` found for type `usize` in the current scope
    //   --> src/common_concepts/variable.rs:66:23
    //    |
    // 66 |     spaces_2 = spaces.len();
    //    |                       ^^^ method not found in `usize`
}