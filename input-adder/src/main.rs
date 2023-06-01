use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let number = input.trim().parse().expect("Invalid number");

    for i in 1..=number {
        println!("i = {}", i);
    }
}
