pub fn if_not_assign_data_type() {
    let guess1: u32 = "42".parse().expect("這不是數字！");
    println!("guess1 = {guess1}");
    // Output:
    // guess1 = 42

    // let guess2 = "42".parse().expect("這不是數字！");
    // println!("guess2 = {guess2}");
    // Output:
    //   |
    // 5 |     let guess2 = "42".parse().expect("這不是數字！");
    //   |         ^^^^^^
    //   |
    // help: consider giving `guess2` an explicit type
    //   |
    // 5 |     let guess2: /* Type */ = "42".parse().expect("這不是數字！");
    //   |               ++++++++++++
}

pub fn scalar_types() {
    // integer
    let integer1 = 1;
    let integer2 = -2;
    let integer3: u64 = 6; // assign data type
                           // let integer4: u32 = -1; // The literal `-1` does not fit into the type `u32`

    // float
    let float1 = 3.0;
    let float2: f32 = 4.0;

    // boolean
    let boolean1 = true;
    let boolean2: bool = false; // assign data type

    // char
    let char1 = 'z';
    let char2: char = 'ℤ'; // assign data type
    let heart_eyed_cat = '😻';
}

pub fn compound_types() {
    // tuple type
    let tuple1: (i32, f64, u8) = (500, 6.4, 1); // assign data type
    let tuple2 = (500, 6.4, 1);
    let tuple3 = (500, 6.4, 1);
    let (x, y, z) = tuple3;

    println!("tuple1: {},{},{}", tuple1.0, tuple1.1, tuple1.2);
    println!("tuple2: {},{},{}", tuple2.0, tuple2.1, tuple2.2);
    println!("tuple3: {},{},{}", x, y, z);

    // array type
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    // let error_index = array[9]; // index out of bounds: the length is 5 but the index is 9
    println!("array index first: {first}");
    println!("array index second: {second}");
    // println!("array index error_index: {error_index}")

    let array_assign_data_type: [i32; 5] = [1, 2, 3, 4, 5];
    let array_create_default = [3; 5];
}
