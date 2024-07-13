#![allow(unused)]

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0.0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

// Error handling techniques in rust
fn main() {
    // Aproach 1
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // Approach 2
    // Handles operations which can succeed with Ok, else it will return an error
    // enum Result<T, E> {
    //     Ok(T),  // Represents a value
    //     Err(E), // Represents an error
    // }
    // Useful to retrieve why something failed

    // let result = divide(34.5, 0.5);
    // match result {
    //     Some(x) => println!("Some value = {x}"),
    //     None => println!("Cannot divide by 0 you stupid"),
    // }

    let result = divide_result(10.0, 10.0);
    match result {
        Ok(result) => println!("Result = {}", result),
        Err(err) => println!("Error is {}", err),
    }
}
