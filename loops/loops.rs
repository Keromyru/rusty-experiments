fn main(){
  // a literal infinite loop!
  loop {
    println!("again!");
  }
  // loops run until explicitly to stop
  // break keyword will stop the loop execution
  // continue will skip over the rest of the code and restart iteration

  // this is helpful if there is a task you know will fail until it suceeds
  // like waiting for a thread to finish its execution
  let mut counter = 0;

  // loops also have return?!

  let result = loop {
    counter += 1;

    if counter == 10 {
      // you can return the value after the break like this
      // super cool 
      break counter * 2;
    }
  };
  // this would print to 20
  println!("The result is {result}");

  // in the case of several nested loops continue and break only 
  // apply to the innermost loop
  // though there are loop labels on a loop that you can then use with
  // break and continue statements, loops labels must begin with single quotes
  // example: 'first_loop 
  
  // this the remaining will always reset as the outermost loop loops
  // the break will only break the inner loop while
  // the break 'counting_up; will break the labelled loop
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  // while loops! control variable to be eval'ed, until false keep going
  let mut number = 3;
  while number != 0 {
    println!("{number}!");

    number -= 1;
  }
  println!("LIFTOFF!!!");

  // using while loop to iterate over a collection
  // BUT THIS IS BAD BECAUSE ITS ERROR PRONE OBVOUISLY
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
  }
  //INSTEAD use for loop
  for element in a {
    println!("the value is: {element}");
  }

  // because for loops are very safe and concise they are used for things that aren't looping through a collection
  // to do this they use a range
  // a range from 1 to 4 inclusive reversed with the .rev() function
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}