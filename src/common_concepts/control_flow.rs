const VAR1: i32 = 5;
const VAR2: i32 = 7;
const CONDITION1: bool = VAR1 + VAR2 == 10;
const CONDITION2: bool = VAR1 + VAR2 != 10;

pub fn ctrl_flow_if() {
    if CONDITION2 { println!("Correct!!") };
    // Output:
    // Correct!!
}

pub fn ctrl_flow_if_else() {
    if CONDITION1 { println!("Correct!!") } else { println!("Error!!") }
    // Output:
    // Error!!
}

pub fn ctrl_flow_if_else_if() {
    if CONDITION1 { println!("CONDITION1 Correct!!") } else if CONDITION2 { println!("CONDITION2 Correct!!") } else { println!("Error") };
    // Output:
    // CONDITION2 Correct!!
}

pub fn ctrl_flow_loop() {
    loop {
        println!("Loop Print");
    }
    // "command + c" can stop printing.
}

pub fn ctrl_flow_loop_break() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Loop Print: {count}");
        if count == VAR2 {
            break;
        }
    }
    // Output:
    // Loop Print: 1
    // Loop Print: 2
    // Loop Print: 3
    // Loop Print: 4
    // Loop Print: 5
    // Loop Print: 6
    // Loop Print: 7
}

pub fn ctrl_flow_while() {
    let mut count = 1;
    while count <= VAR1 {
        println!("while - count at :{count}");
        count += 1;
    }
    // Output:
    // while - count at :1
    // while - count at :2
    // while - count at :3
    // while - count at :4
    // while - count at :5
}

pub fn ctrl_flow_for() {
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("element: {element}");
    }
    // Output:
    // element: 10
    // element: 20
    // element: 30
    // element: 40
    // element: 50
}