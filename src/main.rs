use std::io::{stdin,stdout,Write};
use std::io::prelude::*;

fn main() {
    println!();


    startMenu();
}

fn startMenu() {
    let mut looping: bool = true;
    let mut input = String::new();

    while looping {
        println!("==== Pong in Rust, on the terminal, because dependencies are crazy ====");
        println!("==== ---------------------- By Trevor Stahl ---------------------- ====");
        println!("!!! -Please adjust your terminal screen so you only see this menu - !!!");
        println!("-------- that is, 'Pong in Rust ...' should be at the top -------------");
        println!("-----------------------------------------------------------------------");
        println!("-----------------------------------------------------------------------");
        println!("----- and 'BOTTOM' should be at the bottom, just above your input -----");
        println!("-----------------------------------------------------------------------");
        println!("------ to loop this and test, enter anything other than 'done' --------");
        println!("-----------------------------------------------------------------------");
        println!("--------------------- once set up, enter 'done' -----------------------");
        println!("------------------------------ BOTTOM ---------------------------------");
        let _=stdout().flush();
        stdin().read_line(&mut input);
        if input == "done\n" {
            looping = false;
        }
    }
}