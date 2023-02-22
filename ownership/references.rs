fn main(){
  // ** References and Borrowing **

  // instead of returning a tuple with the value we passed and a new value
  // we can provide a reference to the data we do not want to have its ownership taken
  // A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; 
  // that data is owned by some other variable. 
  // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
  
  // here is how to defined and use a calculate_length function with a reference to an object as a parameter
  // instead of taking ownership of a value 
  // the all powerful ampersand
  let s1 = String::from("hello");
  // this lets us use a reference to s1 instead of ownership! 
  // just a pointer to s1
  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);

  // RANDOM NOTE there is such a thing as deferencing using * ; that will be discussed later

  // the action of creating a reference is called borrowing 
  //  As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
  // if we try to modify something we are borrowing it doesn't work
  // let s = String::from("hello");
  // change(&s); // ruin this for an error
  // the error implies there is `&mut String` instead of just &String O_O
  let mut s2 = String::from("hello");

  change2(&mut s2);
  // NOTE there is one big restriction on mut references, there can be no other references to that mutable value
  // so this code:

  // let mut s = String::from("hello");
  // let r1 = &mut s;
  // let r2 = &mut s;
  // println!("{}, {}", r1, r2);

  // fails because there cannot be more than one mutable reference to a value
  // error says you cannot borrow a mutable value more than once
  // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion
  // he benefit of having this restriction is that Rust can prevent data races at compile time
  // data race is similar to a race condition and happens when these three conditions occur:
  // - two or more pointers access teh same data at the same time.
  // - at least one of the pointers is being used to write to the data.
  // - there's is no mechanism being used to synchronize access to the data.
  // these cause undefined behaviour and are hard to track down at run time; 
  // this problem is mitigated by not allowing that to happen in rust

  // curly braces create new scopes allowing for multiple mutable references just not simultaneous ones
  let mut s = String::from("hello");
  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.
  let r2 = &mut s;

  // this commented out code causes problems because it has immutable references which is okay
  // but taking a mutable reference is not okay
  
  // let mut s = String::from("hello");
  // let r1 = &s; // no problem
  // let r2 = &s; // no problem
  // let r3 = &mut s; // BIG PROBLEM
  // println!("{}, {}, and {}", r1, r2, r3);

  // We also cannot have a mutable reference while we have an immutable one to the same value
  // users expect the data to not suddenly change and several immutable values is okay
  // it doesn't interfere with anyone just reading the values
  
  // this is the correct way to use immutable and mutable references
  let mut s3 = String::from("hello");

  let r1 = &s3; // no problem
  let r2 = &s3; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s3; // no problem
  println!("{}", r3);
  // this works because the scopes don't overlap so its allowed
  // can be frustrating but is helpful to not have esoteric bugs at run time

  // the compiler will also try to stop dangling references/pointers
  let reference_to_nothing = dangle(); // this will error out because there is no value
  // returning a reference to a string that no longer exists is a problem! 

  // rules of references 
  // - at any given time you can have either one mutable ference or any number of immutables references
  // references but always be valid
  
}

fn dangle() -> &String { // dangle returns a reference to a String
  let s = String::from("hello"); // s is a new String
  &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
// the solution is to return the string directly
fn no_dangle() -> String {
  let s = String::from("hello");
  s
}

fn change2(some_string: &mut String) {
  some_string.push_str(", world");
}

// note the typing is a reference to a string not just a string value
fn calculate_length(s: &String) -> usize { // s is a reference to a String
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }