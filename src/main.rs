#![allow(unused)]

/*
References: Enable you to borrow values without taking ownership
Types:
1. Immutable References
2. Mutable References
Create a reference by simply adding "&"
*/

fn calculate_len(s: &String) -> usize {
    s.len()
}

// You can either have one mutable reference or many immutable references
fn main() {
    let mut x: i32 = 5;
    // creating immutable reference to x
    let r = &mut x; // transferring whole object
                    // let b = &x;
    *r += 1;
    *r += 1;
}
