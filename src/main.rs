/*brings in the input/output library into scope 
from the standard library*/
use std::io;

//main function entry point into a rust program
fn main(){

    /*macro println! prints a string argument to 
    the screen*/
    println!("Guess the number!");

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
    io::Result of read_line*/
        .expect("Failed to read line");

    /*{} is a placeholder that represents the 
    values in order after the format string, so here
    {} will be replaced with the value of 'guess'
    that comes from the user input 
     */
    println!("You guessed: {}", guess);
}
