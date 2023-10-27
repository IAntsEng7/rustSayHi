pub fn immutable_test(){
    let var_1 = "Only can assign once!!";
    let mut var_2 = "Try to assign multiple times";

    println!("1. var_1: {var_1}");
    println!("1. var_2: {var_2}");

    // var_1 = "try";
    println!("2. var_1: Cannot assign a new value to an immutable variable more than once.");
    var_2 = "Assign second times.";
    println!("2. var_2: {var_2}")
}