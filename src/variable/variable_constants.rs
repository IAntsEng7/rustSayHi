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