#![allow(unused)]

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Each variable has it's own owner
// fn main() {
//     let s1 = String::from("Rust");
//     let len = calculate_length(&s1);
//     println!("Length of {} is {}", s1, len);
// }

// At any given time, there could only be one owner of a variable
fn main() {
    let s = String::from("RUST");
    let s2 = s;

    // println!("{}", s2);
}

// hello copilot
