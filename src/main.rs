use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("+++++++++++++++++++++++++++");
    println!("Welcome to guess the number");
    println!("+++++++++++++++++++++++++++");
    println!("\n");

    // Generating a random number on the current local thread
    // The ..= is a "rango" operator. It take a start and an end. Both inclusive. start..=end.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Runs an infinite loop
    loop 
    {
        println!("Guess the number: ");

        // In Rust, variables are immutable by default, 
        // which means that you have to explicitly say if a variable can be mutated if you want to
        // Using String::new() will "construct" a new empty string. Since String is a type,
        // You need to construct it using "new()." This is similar to something like 'String guess = new String().'
        // In Rust, the "::" sign indicated the "assoiated function" of a type.
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess) // Reads the user's input
            .expect("Failed to read the message"); // Prints this line when there is an error

        println!("Your guess: {guess}");

        // Converts the string to an int 32
        // Instead of using "expect" here, the match statement is used.
        // Since parse returns the Result enum which can either return the result or an error, 
        // we can handle the error this way using the match statement.
        // We tell it to ignore the user if he typed a non number, but return the result if it is a number.
        // This is how you can handle errors in Rust
        let guess: u32 = match guess.trim().parse() {
            // The num is the result that the "parse" function produces, and it will be the value of guess,
            // if all goes well
            Ok(num) => num,
            // Ignores the input if the user writes a non number.
            // The "_" is a catchall errors operator, which tells the program to continue skip over whatever the error might be
            Err(_) => continue,
        };

        // Comparing the user's input and the randomly generated number
        // Any type in Rust that can be compared will have a "cmp" or compare method.
        // It will return an Ordering enum which will have the three possible outcomes with comparing two values.
        // The "cmp" method will take a refrence to whatever variable you want to compare it to
        match guess.cmp(&secret_number)
        {
            // Checks if the secret number is less than, greater than or equal to the guess
            Ordering::Less => println!("TOO SMALL! Like your PP!!!!!!!\n"),
            Ordering::Greater => println!("Too big! Like MY PP!!!!!!!!!\n"),
            Ordering::Equal =>
            {
                // Breaking out of the loop when the user correctly guesses the number
                println!("Congratulations! You WIN\n");
                break;
            },
        }
    }
}
