// mods
mod nums;
mod args;

// imports
use std::io;
use clap::Parser;
use args::Arguments;

fn main() {
    println!("Input a number to convert...");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error: failed to read line");

    let num: i32 = input
        .trim()
        .parse()
        .expect("Error: input is not an integer");

    let res = nums::to_decimal(num);

    println!("The result is {}", res);
}
