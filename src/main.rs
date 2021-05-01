#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::{self, Write};

mod step_0;
mod step_1;

fn rep_for_step(input: &String, step: u32) -> &str {
    match step {
        0 => step_0::rep(&input),
        1 => step_1::rep(&input),
        _ => step_1::rep(&input),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let step_arg = args.get(1);
    let step_arg_num = match step_arg {
        Some(x) => x.parse::<u32>().unwrap(),
        _ => 0,
    };

    loop {
        print!("user> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("{}", rep_for_step(&input, step_arg_num));
    }
}
