pub fn function_test() {
    // no parameter
    let no_param = no_param_test();
    println!("no_param return: {no_param}");
    // Output:
    // no_param return: no_param_test return

    // with parameter
    let with_param = with_param_test(5, 7);
    println!("with_param return: {with_param}")
    // Output:
    // with_param return: 12

}

fn no_param_test() -> String {
    "no_param_test return".to_string()
}

fn with_param_test(add1: u32, add2: u32) -> u32 {
    return add1 + add2;
}