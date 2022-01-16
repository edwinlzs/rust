use std::io; // import standard input/output library
use std::cmp::Ordering; // Ordering enum type with variants Less, Greater, Equal
use rand::Rng; // Rng trait defines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    /*
        thread_rng() gives the particular random number generator to use (local to current
        thread of execution and seeded by OS)
        gen_range() method on the generator takes a range expression start..end (exclusive of
        upper bound but inclusive of lower) (we could alternatively use 1..=100) 
    */
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // creates mutable variable bound to a new empty instance of String

        io::stdin() // call stdin() function from io library
            /*
                calls read_line() method on standard input handle to get user input
                and store it in guess. The & indicates that this argument is a reference.
            */
            .read_line(&mut guess)
            /*
                io:Result returned by the user input has an expect() method
                that causes the program to crash and display the input message
                if the Result enum has an Err variant. If expect() is not called, the program
                will compile but gives a warning that the Result value hasn't been used,
                hence the program hasn't handled a possible error.
            */
            .expect("Failed to read line");  

        /*
            Rust is shadowing the previous value of guess with a new one here.
            Shadowing lets us reuse the guess variable name rather than forcing us to
            create two unique variables
        */
        let guess: u32 = match guess // RHS guess refers to the original guess variable.
                            .trim() // eliminates any whitespace at the beginning and end
                            .parse() // string method that parses it into some kind of number
                            {
                                Ok(num) => num,
                                Err(_) => continue,
                            }; // we use match expression instead of expect call to handle the error
                            // .expect("Please type a number!"); 

        println!("You guessed: {}", guess); // curly brackets {} are placeholders, based on position

        /*
            cmp method compares 2 values. match expression decides what to do next based
            on which variant of Ordering was returned from the call to cmp with the values
            guess and secret_number
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
