pub fn coba_unwrap() {
    let some_value = Some(42);
    let value = some_value.unwrap();
    println!("Unwrapped Value: {}", value);

    // let none_value: Option<i32> = None;
    // let value = none_value.unwrap(); // <-- this will raise panic
    // println!("Value: {}", value);
}

pub fn safer_approach_than_unwrap() {
    let some_value = Some(42);
    let value = match some_value {
        Some(value) => value,
        None => 0,
    };
    println!("Value: {}", value);

    let none_value: Option<i32> = None;
    let value = match none_value {
        Some(value) => value,
        None => 0,
    };
    println!("Value: {}", value);
}
