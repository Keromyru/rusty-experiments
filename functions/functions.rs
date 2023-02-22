
fn main(){
  
  println!("Hello, world!");
  // functions in rust by convention are written in snake case! 
  another_function(5);
  print_labeled_measurement(5, 'h');

  // function bodies are made up of statements that can optionally end in an expression
  // these ending expressions haven't been seen yet, though we've seen expressions before
  // also statements do not return a value so something like:
  // let x = (let y = 6);
  // will result in an error because this doesn't have a return value
  // The let y = 6 statement does not return a value, 
  // so there isn’t anything for x to bind to. This is different from what happens in other languages, 
  // such as C and Ruby, where the assignment returns the value of the assignment. In those languages, 
  // you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.
  // calling a function is an expression, calling a macro is an expression, a new scope block with curly braces is an expression
  let y = {
    let x = 3;
    // note the missing semi colon on x + 1, adding a semi colon turns it into an statement
    // without a semi colon this is an expression and will return a value, hence why it 
    // returns out of the braces to bind y to 4 (?)
    x + 1 // no semi colon, the implicit function return statement
  };
  println!("The value of y is: {y}");

  // function return values are not named but they need to have their type declared after an arrow
  // the return value is synonymous with the final expression in a function block see function five()
  let z = five();
  println!("The value of x is: {z}");
  // you can use return keyword with a value to return early from a function 

  let x = plus_one(5);
  // this print statment should be 6 and it will be
  // you can error out this and put a semi colon on the expression and it will error saying it found () not i32
  println!("The value of x is: {x}");

}

// another example of returning an expression
fn plus_one(x: i32) -> i32 {
  x + 1
}

// example of typing the return value! 
fn five() -> i32 {
  5 // note no semi colon! to indicate the last expression in the function, the last thing to be returned
}

// this was defined before the call in main however rust doesn't care
// Rust doesn’t care where you define your functions, only that they’re 
// defined somewhere in a scope that can be seen by the caller.
fn another_function(x: i32) {
  println!("Another function.");
  println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}