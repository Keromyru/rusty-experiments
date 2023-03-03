/*
  Rust has an extremely powerful control flow operator called match that allows comparison of a value
  against a series of patterns and then executes code based on which pattern matches. 
  Patterns can be:
    - literals
    - variable names
    - wildcards
    - many other things
  The power is from the expressiveness of hte matching and that the compiler will ensure that all cases
  are handled. 

*/

enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

/*
  coin example to show a functon that has a match expression that takes a coin and returns a body
  Takes in one of the coin variants and returns a u8 representing the value of said coin
*/
enum Coin {
  Penny,
  Nickel,
  Dime,
  // Quarter,
  Quarter(UsState); // this is a quarter with a state associated with it
}
/*
  another useful feature of match is that they can bind to the parts of the values that match the pattern.
  this is how extraction from enum variants can be done by adding another emum with the states the 
  coins were produced in.
  this way lets it return what state the quarter is from if it matches that arm of the match expression

*/

fn value_in_cents(coin: Coin) -> u8 {
  // the code assocaited with each arm is an expression and the resultant value of the expression
  // in the matching arm is the value that gets returned for the entire match expression; which in this case
  // the entire function returns the match. 
  match coin {
    // when the match is short there doesnt need to be curly braces
    // Coin::Penny => 1,
    // however for multiple lines curly braces are needed as below
    // also note that there is no semi colon after the value being returned, similar to function's last statement
    // being the return value when there is no semi colon.
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // Coin::Quarter => 25,
    Coin::Quarter(state) => {
      // this lets us access the inner state vaue out of the quarter
      println!("State quarter from {:?}!", state);
      25
    }
  }
}

/*
  Recall before with the inner T we wanted to access from Option<T>; Some(T) would require us to access the inner value
  Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. 
  If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
*/

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1), // this will take the inner value so Some and add 1 to it
    // creating and returning the new value
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

/*
  there are also catch all values and execute a default behaviour. Say a game where you roll a particular number
  and it does something. If you roll a 3 you get a fancy hat, if you roll a 7 you lose your fancy hat, and if you roll
  any other number you move that many spaces. 
*/
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // this is a catch all that we want to use the value for
    other => move_player(other), // of course this is last as it is a catch all
    // there is another catch all value called _ which is a wildcard and will match any value
    // but does not bind to any data. It also removes the Rust warning about a unused variable
    // if we change the rules to having to reroll any other value this catch would become
    // _ => reroll(), // this is also equally exhaustive, we are covering all possible values
    // just ignoring the value and rerolling

    // we could also change the rules again and say that nothing else happens if you roll any other number
    // than 3 or 7. that would look like 
    // _ => (), // this is a unit value, which is a value that means nothing 
    // essentially any other value, there is no code to run
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    println!("Hello, world!");
}
