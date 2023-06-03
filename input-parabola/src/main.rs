use std::io;

fn main() {
    let mut array: [i32; 3] = [0; 3];

    for (_, element) in array.iter_mut().enumerate() {
        let mut input = String::new();

        println!("Enter a number:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number = input.trim().parse::<i32>().expect("Invalid number");

        *element = number;
    }

    println!("{}x^2 + {}x + {}", array[0], array[1], array[2]);
}
