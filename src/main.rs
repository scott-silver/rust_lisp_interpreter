#[macro_use]
extern crate lazy_static;

use std::io::{self, Write};

mod step_0;
mod step_1;

fn main() {
    loop {
        print!("user> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("{}", step_0::rep(&input));
    }
}
