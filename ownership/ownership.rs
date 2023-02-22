fn main(){
  // scoping is similar in other languages in the sense that a block scope is valid then it is once the scope is done

  // the String type is more complex and allows for non string literals to be used
  // like user input can be created in a string type using String::from() note :: is how to access methods
  // let s = String::from("hello");
  // this type of string can also be mutated
  let mut s = String::from("hello");

  s.push_str(", world!"); // push_str() appends a literal to a String

  println!("{}", s); // This will print `hello, world!`

  // once something is out of scope it will be freed 
  // so:
  {
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
  }                               // this scope is now over, and s is no longer valid
  // when something goes out of scope Rust calls a special function called drop
  // this seems simple but is very difficult when things becomes more complex
  let x = 5;  
  let y = x; // both these values are pushed onto the stack both are 5
  // s1 has a pointer, length and capacity that is stored on the stack
  // (note length and capacity are different, but not relevant here)
  // while the value (hello) is stored in the heap  
  let s1 = String::from("hello");
  let s2 = s1; // this only copies the info on the stack NOT the value on the heap
  // println!("{}, world!", s1); // run this to get the error

  // this causes a problem with rust calling drop on s1 and s2 as it will be a 
  // double free error, and can corrupt memory
  // Rust in this case will invalidate s1 and s2 will own that value on the heap
  // trying to use s1 after the fact will cause a compiler error! 
  // this seems like a shallow copy but it is instead a move
  // data of s1 was moved to s2, this is because s1 is invalidated after the move
  // then once they go out of scope, s1 invalidated, s2 will be dropped, no double free error
  // Rust will never create deep copies and automatic copies are mostly moves instead

  // to copy both stack and heap of a String a common method of clone can be used
  let s3 = String::from("hello");
  let s4 = s3.clone(); // this copies stack and heap data

  println!("s3 = {}, s4 = {}", s3, s4);

  // going back to the integer example, integers have a specific size and their values are stored on the stack, 
  // so copies of them are very quick to make. clone won't do anything different than the default
 
  // there is a annotation called Copy that can be placed on types that are stored on the stack
  // such as integers have. ** If a type implements the Copy trait, variables that use it do not
  // move, but rather are trivally copied, making them still valid after assignment to another variable
  // This will not be allows if a type or any of its parts, has implemented the Drop trait 
  // (there is more on how to add a Copy annotation to a type later)
  // a rule of any, any group of scalar values can implement Copy, and nothing that requires 
  // allocation or is some form of resource can implement Copy.

  // Things that can implement Copy
  // - All the integer types, such as u32.
  // - The Boolean type, bool, with values true and false.
  // - All the floating-point types, such as f64.
  // - The character type, char.
  // - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

  // ** Ownership and functions **
  
  // The mechanics of passing a value to a function are similar to those when assigning a value to a variable. 
  // Passing a variable to a function will move or copy, just as assignment does
  
  let s5 = String::from("hello");  // s5 comes into scope

  takes_ownership(s5);             // s5's value moves into the function...
                                  // ... and so is no longer valid here

  let x2 = 5;                      // x2 comes into scope

  makes_copy(x2);                  // x2 would move into the function,
                                  // but i32 is Copy, so it's okay to still
                                  // use x2 afterward
  // at the next curly brace the value goes out of scope (end of main function) x2 goes out of scope, then s5. 
  // But because s5's value was moved, nothing special happens.

  // however return values can transfer ownership
  let s6 = gives_ownership(); // gives_ownership moves its return value into s6

  let s7 = String::from("hello");     // s7 comes into scope

  let s8 = takes_and_gives_back(s7);  // s7 is moved into takes_and_gives_back, which also
  // moves its return value into s8
  // at the next curly brace (end of main function) s8 goes out of scope and is dropped. s7 was moved, so nothing happens. 
  // s6 goes out of scope and is dropped.

  // ownership of a variable follows the same pattern every time assigning a value to another variable moves it. 
  // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless 
  // ownership of the data has been moved to another variable.

  // it can be tedious to keep returning values we want ownership back from
  // functions can return tuples to facilitate returning the value that 
  // transfered ownership as well as a new value
  let (s9, len) = calculate_length(s1);

  println!("The length of '{}' is {}.", s9, len);
  // But this is too much ceremony and a lot of work for a concept that should be common. 
  // Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {  // gives_ownership will move its
  // return value into the function that calls it
  let some_string = String::from("yours"); // some_string comes into scope

  some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}