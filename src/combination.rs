use std::io;

pub fn main() {
    let mut input = String::from("");

    println!("\n\nHow many generations do you want to run? :");
    input = get_input();

    println!("\nRunning {} generations...", input.trim());
}

fn get_input() -> String {
    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}