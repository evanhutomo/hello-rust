mod geometry;
mod try_unwrap;
use try_unwrap::coba_unwrap as cu;

mod try_expect;
use try_expect::coba_expect as ce;

mod try_error;
use try_error::read_file;

mod try_thread;
use try_thread::coba_thread as ct;

mod try_async;
use try_async::run_async as ra;

use ferris_says::say;
use std::{
    collections::btree_map::Keys,
    io::{stdout, BufWriter, Stdout},
};

use geometry::shapes::Rectangle;
use geometry::utils::distance;
use rand::Rng;

fn main() {
    // try async
    ra();

    // try thread
    ct();

    // try error
    let result = read_file("example.txt");
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => eprintln!("Error: {}", error),
    }

    // coba unwrap
    cu();
    ce();

    // random
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..9);
    println!("Random number: {}", random_number);

    // local variables
    let message = String::from("Hello");
    let mut writer = String::new();

    // function calls
    test_tuple();
    test_vector();
    test_match();

    test_option();
    let result_fn_option = fn_option(vec![1, 2, 3, 4, 5], 3);
    match result_fn_option {
        Some(number) => println!("Found number: {}", number),
        None => println!("Number not found"),
    }

    test_result();
    let result_fn_result = fn_result(10, 2);
    match result_fn_result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }

    test_struct();

    crustacean();
    iterator_sample();
    print_message(&message);
    append_message(&mut writer, &message);
    println!("Writer: {}", writer);

    let numbers = vec![1, 2, 3, 4, 5];
    let result = find_number(numbers, 3);
    match result {
        Some(number) => println!("Found number: {}", number),
        None => println!("Number not found"),
    }

    let rect = Rectangle::new(10.0, 20.0);
    println!("Area: {}", rect.area());

    let dist = distance(1.0, 2.0, 3.0, 4.0);
    println!("Distance: {}", dist);
}

fn fn_option(numbers: Vec<i32>, target: i32) -> Option<i32> {
    for &number in &numbers {
        // this is a reference to the number so that we don't take ownership of the number
        if number == target {
            return Some(number);
        }
    }
    None // return None if the number is not found
}

fn fn_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Division by zero"));
    } else {
        return Ok(a / b);
    }
}

fn test_result() {
    let value: Result<i32, String> = Ok(3);

    match value {
        Ok(1) => println!("One"),
        Ok(2) => println!("Two"),
        Ok(3) => println!("Three"),
        Ok(_) => println!("Some other number"), // Matches any other `Ok` value
        Err(message) => println!("Error: {}", message), // Matches `Err` and binds the error message to `message`
    }
}

fn test_option() {
    let value: Option<i32> = Some(3);

    match value {
        Some(1) => println!("One"),
        Some(2) => println!("Two"),
        Some(3) => println!("Three"),
        Some(_) => println!("Some other number"), // Matches any other `Some` value
        None => println!("None"),                 // Matches `None`
    }
}

fn test_match() {
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 => println!("Four or Five"),
        _ => println!("Something else"),
    }
}

fn test_vector() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    println!("{:?}", numbers);
    let first = numbers[0];
    let second = numbers[1];
    println!("First: {}, Second: {}", first, second);
    let third = numbers.get(9);
    match third {
        Some(number) => println!("Third: {}", number),
        None => println!("No third number"),
    }
    for number in &numbers {
        println!("{}", number);
    }
    numbers.pop();
    println!("{:?}", numbers);
}

fn test_tuple() {
    let tuple: (i32, f64, char) = (42, 6.12, 'j');
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn find_number(numbers: Vec<i32>, target: i32) -> Option<i32> {
    for &number in &numbers {
        // this is a reference to the number so that we don't take ownership of the number
        if number == target {
            return Some(number);
        }
    }
    None // return None if the number is not found
}

fn crustacean() {
    let stdout: Stdout = stdout();
    let message: String = String::from("HTS H nya apa, H nya HANCOK");
    let width: usize = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn print_message(message: &String) {
    // &String means immutable reference to a String
    println!("{}", message);
}

fn append_message(writer: &mut String, message: &String) {
    // &mut String means mutable reference to a String
    writer.push_str(message);
}

fn iterator_sample() {
    let message = String::from("Hello");
    // `chars()` returns an iterator over the characters of the string
    let mut chars = message.chars();
    // Using the `next()` method to get each character from the iterator
    println!("{:?}", chars.next()); // Some('H')
    println!("{:?}", chars.next()); // Some('e')
    println!("{:?}", chars.next()); // Some('l')
    println!("{:?}", chars.next()); // Some('l')
    println!("{:?}", chars.next()); // Some('o')
    println!("{:?}", chars.next()); // None (iterator is exhausted)
}

fn test_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("user1"),
        email: String::from("aa@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    // let user2 = User {
    //     username: String::from("user2"),
    //     ..user1
    // };

    let point = Point { x: 3, y: 4 };
    println!("x: {}, y: {}", point.x, point.y);
    print!("User1: ");
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    // print!("User2: ");
    // println!("username: {}, email: {}, sign_in_count: {}, active: {}", user2.username, user2.email, user2.sign_in_count, user2.active);
}
