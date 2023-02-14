// variables 
fn main() {
    // let x = 5;// this is not mutable and reassigning later throws an error
    let mut x = 5; // this lets it be mutable and not throw an error
    // also shows the intent to change this variables value later
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants are always immutable and cannot be created at runtime and must have their
    // type delcared and constants may be set only to a constant expression:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // it is convention to write constants in all caps with _s 
    // the complier is able to evaluate a limited set of operations at compile time
    // these can be global or scoped but they are valid for the entire run time
    // it also declares that constants will not change and if needs to be updated it only should be updated
    // in the one spot/declaration

    // like in the guessing game variables can be shadowed where it can be redeclared with the let keyword and the same name
    let y = 5; // first binding
    // second binding turns it to 6
    let y = y + 1;

    // curly braces creates a inner scope
    {   // this shadows the first y var and changes its value to 12
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12 is the result
    } // but when the inner scope ends, the y value outputs 6
    println!("The value of y is: {y}"); // 6 is the result 

    // this would result in an error if we used mut
    // because we should still be trying to reassign without using let which isnt allowed
    // but using let and shadowing there can be a couple transformations then the value
    // is immutable again
    // let is also creating a new variable with the same name but different value

    // trying to use mut and change the variable results in an error because its the wrong type
    // (though i think this can change if we type it? like the in game example)
    let spaces = "   ";
    let spaces = spaces.len();
    // in this example we want the number of spaces, so using let lets us keep the descriptive name
    // space but transform the data how we want
}