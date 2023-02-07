use std::io;
// use state to use a package and :: to use something from within that package
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    // thread rng creates a rng generator that is local and
    // is seeded by the operating system.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // this method is defined by the Rng trait, the gen_range takes a
    // range expression in the form of start..=end inclusive on upper and lower bounds

    // using crate methods is unclear but to see documentation can go
    // cargo doc --open to build documentation locally and open it in a browser

    println!("The secret number is: {secret_number}");

    println!("Please input your guess");

    // let variable declaration immutable
    // however mut allows for mutability
    let mut guess = String::new(); // creating a new mutable empty string

    // ooo fluent syntax, no semicolon means its still the same expression
    // this also returns a result value; which is an enum
    // which will success and error info Ok and Err respectively
    // if you don't use expect, which returns Ok's value, it could be an 
    // Err and you need to handle it, compiler says so

    // for this example the Ok value is the user input as number of bytes
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // note passing in mut guess not JUST guess as it would be immutable (even tho its declared as mutable?) 
    // and passing in a mutable version of guess
    // ensuring it is passed as a reference using &
    
    // this is shadowing the old guess with a new one, which lets us reuse the old variable name
    // without having to create a new one but allows us to change the type but makes it clear this is happening
    // trim is important here as it will get rid of leading and trailing whitespace which, if it is a number
    // allow it to be compared and converted
    // parse will allow us to parse a value into another. which is noted by the :<type> with the variable name
    // also in this case rust will infer the secret number should be u32 as well
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // the variable can be in the braces or
    // println!("you guessed: {}", guess)
    println!("You guessed: {guess}");

    /// cmp can be used to compare two values, so using it on guess
    // passing in the reference of secret_number

    // match has arms with each arm being a pattern to match against
    // this allows for each pattern to describe a state that we can handle
    // it lookes through each pattern to be matched once matched
    // it will stop looking for a match
    // it then will execute whatever the associated action is 

    // note in this example, our guess is a string while secret number is a number
    // the compiler complains about this
    // so after reading std::in the value of input can be checked and changed to be a number
    // or if that isn't possible, as the user to redo their guess
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
