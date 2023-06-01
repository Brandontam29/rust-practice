use std::io;

fn main() {
    let mut input1 = String::new();

    println!("Enter the first number");

    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read line");

    let x = input1.trim().parse::<i32>().expect("Invalid number");
}
