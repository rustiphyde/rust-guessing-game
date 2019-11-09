/*brings in the input/output library into scope 
from the standard library*/
use std::io;
/* brings in the Ordering comparison type from the 
standard library*/
use std::cmp::Ordering;
/*brings in the random number generator trait 
from the rand crate*/
use rand::Rng;

//main function entry point into a rust program
fn main(){

    /*macro println! prints a string argument to 
    the screen*/
    println!("Guess the number!");

    /* stores a random generated number between 1 
    and 100 into the variable secret_number using the 
    thread_rng method from the rand crate chained to the 
    gen_range method from the same crate
    To get a better understanding of the use of rand
    run cargo doc --open in the terminal*/
    let secret_number = rand::thread_rng().gen_range(1, 101);

    /* shows the random number that was generated for 
    testing purposes, this gets commented out before
    final version is released*/
    println!("The secret number is: {}", secret_number);

    // requests user input
    println!("Please input your guess.");

    /*creates a mutable variable called guess bound to 
    a new empty instance of the growable String type.
    :: indicates that new is an associated function of 
    the String type*/
    let mut guess = String::new();

    /*calls the stdin associated function from the 
    io module and chains the read_line method to it to 
    get input from the user 
    & indicates that the argument is a reference to the 
    guess variable, mut makes it mutable
     */
    io::stdin().read_line(&mut guess)
    /*expect handles the Err variant of  the 
    io::Result enum of read_line*/
        .expect("Failed to read line");

    /*shadows the previous value of guess with a new one
    bound to the expression which takes the original
    guess value minus whitespace from pressing enter
     and parsed into an unsigned 32 bit integer type 
     annotated after the :*/
     /*If a number is not provided the expect method
     of the parse Result fires off*/
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    /*{} is a placeholder that represents the 
    values in order after the format string, so here
    {} will be replaced with the value of 'guess'
    that comes from the user input */
    println!("You guessed: {}", guess);

    /*match expression is made up of arms. An arm 
    consists of a pattern and the code that should be 
    run if the value given to the beginning of the 
    match expression fits that arm’s pattern.*/
    /*cmp compares two values - the start value (guess)
    and the reference value argument(&secret_number)*/
    /*The Ordering enum has the 3 variants that are
    possible when comparing two values - Less, Greater
    and Equal*/
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! The number was {}", secret_number),
        Ordering::Greater => println!("Too big!The number was {}", secret_number),
        Ordering::Equal => println!("You guessed the secret number! Congratulations, you win!"),
    }
}


