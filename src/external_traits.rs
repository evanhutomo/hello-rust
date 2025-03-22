use std::fmt::Debug; // external traits

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    // Using the Debug trait to print the struct
    println!("{:?}", point);

    // Using the Debug trait to print the struct with pretty formatting
    println!("{:#?}", point);
}
