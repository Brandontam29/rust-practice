use std::io;

fn main() {
    loop {
        println!("Enter a number:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Trim leading/trailing whitespace and handle empty input
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue; // Restart the loop to prompt for input again
        }

        // Parse the input into an i32
        let number = match trimmed_input.parse::<i8>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue; // Restart the loop to prompt for input again
            }
        };

        // Input is valid, perform further operations
        println!("Valid number: {}", number);

        // Break out of the loop
        break;
    }

    // Rest of the program execution
}
