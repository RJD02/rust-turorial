#![allow(unused)]

// shadowing is not the same as marking a variable as mutable
fn main() {
    let x = 5;
    let x = x + 2;
    {
        // here
        let x = x + 1;
        println!("Value of x is {}", x);
    } // here
    println!("Hello world");

    println!("Value of x is outside scope is {}", x);
}
