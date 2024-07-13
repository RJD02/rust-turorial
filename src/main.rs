#![allow(unused)]

const YOU_CAN_DECLARE_A_CONSTANT_HERE: i32 = 45;

fn main() {
    println!("Hello, world");
    const Y: i8 = 5; // upper case
                     // constant is immutable throughout it's lifecycle
                     // you cannot mutate it
                     // always have to provide type for it
}
