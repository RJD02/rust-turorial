#![allow(unused)]

fn main() {
    // let age = 15;
    // let result = if age > 10 { "allowed" } else { "not allowed" };
    // println!("{result}")
    // loop {
    //     println!("Hello World");
    // }
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("{result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 2 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
            count += 1;
        }
    }
}
