pub fn coba_expect() {
    let some_value = Some(42);
    let value = some_value.expect("Failed to unwrap the value");
    println!("Expect Value: {}", value);

    // let none_value: Option<i32> = None;
    // let value = none_value.expect("Failed to unwrap the value"); // <-- this will raise panic
    // println!("Expect Value: {}", value);
}
