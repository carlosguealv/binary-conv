// mods
mod nums;

// imports
use std::io;
use std::process;

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

    println!("Do you want to convert to b (binary) or d (decimal)?");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Error: failed to read line");

    // create a variable to store the result of the conversion
    let res: i32;
    if option.trim() == "d" {
        res = nums::to_decimal(num);
    } else if option.trim() == "b" {
        res = nums::to_binary(num);
    } else {
        println!("Error: the inputted option is not valid!");
        process::exit(1);
    }

    println!("The result is {}", res);
}
