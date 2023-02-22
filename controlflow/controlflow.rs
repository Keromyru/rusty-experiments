fn main(){

  // if expressions! 
  let number = 3;
  // the different outcomes of control flow are called arms like match expressions
  if number < 5 {
      println!("condition was true");
  } else { // else is optional if no else and an if statement resolves to false the execution continues
      println!("condition was false");
  }

  if number != 0 {
    println!("number was something other than zero");
  }
  // if statements need to be bools! Or expressions that result in bools
  let number2 = 6;

  if number2 % 4 == 0 {
      println!("number is divisible by 4");
  } else if number2 % 3 == 0 {
      println!("number is divisible by 3");
  } else if number2 % 2 == 0 {
      println!("number is divisible by 2");
  } else {
      println!("number is not divisible by 4, 3, or 2");
  }

  // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
  let condition = true;
  let number3 = if condition { 5 } else { 6 }; 
  
  // this commented out code is an error because the type is mismatched
  // let number3 = if condition { 5 } else { "six" };
  println!("The value of number is: {number3}");


}