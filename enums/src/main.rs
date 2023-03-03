/*
  Enums allow us to define a type by enumerating its possible values.
  And enum encode meaning along with data. A particulary useful enum is
  Option which expresses that a value could be something or nothing.
  Then using pattern matching in the match expression makesit easy to run different code
  for different values of an enum.  if let is also a handy convenient to handle enums in code!
*/
/*
  Enums give us a way of saying a value can be one of a  possible set of values.
  Like saying Rectangle, circle and triangle are all possible shapes. An enum is perfect for this
  Example is the current two IP addresses that we can come across. 4 and 6.
  We can enumerate all possible variants! (which is where enums get their names)
*/
// this is a custom data type that can be used else where in our code.
enum IpAddrKind {
  // old way of doing this
  // V4,
  // V6,
  // this now says that each variant will have associated String values
  // V4(String),
  // V6(String),
  // this allows for data to be attached directly so there is no need for an extra struct
  // each definition now has a constructor as a result of defining the enum this way
  /*
    another advantage over structs is being able to have each variant can have different types
    and amounts of associated data, V4 will always have four numeric components with values between 0 and 255
    We can store V4 like this but keep V6 as a String; this would be impossible with a struct
  */
  V4(u8, u8, u8, u8),
  V6(String),
}

// second iteration where both types take in a string
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
// third iteration
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

/*
  this is a common use case that the stadard library as this implemented
  they instead create structs that take the structs as types for each IP address type
*/
// struct Ipv4Addr {
//   // --snip--
// }
// struct Ipv6Addr {
//   // --snip--
// }
// enum IpAddr {
//   V4(Ipv4Addr),
//   V6(Ipv6Addr),
// }
// enums can contain any type of data, including another enum
// and if we implement our own it wont conflict because it isn't brought into scope from the standard library

/*
  Here an another example of an enum whoes variants each store different amount and types of data
*/

enum Message {
  Quit, // no data associated with this variant
  Move { x: i32, y: i32 }, // has named fields like struct does
  Write(String), // includes a single string
  ChangeColor(i32, i32, i32), // includes three i32 values
}
// this is kinda similar to defining different struct definitions except:
// it doesnt use the `struct` keyword
// the variants are grouped together under the `Message` enum type
// The follow structs could hold the same data as the enum above
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
// but being different types we couldnt define a function that takes and correctly outputs for each of these messages
// and similar to adding methods on structs we can add methods on enums
impl Message {
  fn call(&self) {
      // method body would be defined here
      // using self to get the value that we called the method on.
  }
}
let m = Message::Write(String::from("hello"));
m.call();
// another useful enum is called Option
/*
  The advantages of using the Option enum over null values
    For example, if you request the first item in a non-empty list, you would get a value. 
    If you request the first item in an empty list, you would get nothing. 
    Expressing this concept in terms of the type system means the compiler can check whether 
    you’ve handled all the cases you should be handling; this functionality can prevent 
    bugs that are extremely common in other programming languages.
  Rust doesn't have the Null value like lots of languages do. this is because it becomes tricky
  to use null as a not null value, and every value can be null or not null
  the concept of null is important, something is missing or not there for some reason
  but it is not a value in and of itself... so the option comes in
*/
/*
  it represents the idea a value could be something or nothing
*/
enum Option<T> {
    None,
    Some(T),
}
// how it is used
let some_number = Some(5); // these first two do not need a type as it can be inferred
let some_char = Some('e');
let absent_number: Option<i32> = None; // but this does need a type as it cannot be inferred
// so we need to specify the type of the None value

/*
  using the option means we can use values that may or may not be there and the compiler 
  wlll ensure we handle both cases and stop us from using values in a way that may not make sense
  This code: 
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

  causes an error because it cannot add i8 to an Option<i8> 
  instead any T (so pretty much everything) used with option
  needs to be converted into its type to use it. 
  This catches the most common error, assuming something isn't null when it is
  In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>
  then when you are using that value you must explicitly handle the case when the value is null and the compiler will help you
  ensure you don't miss any cases; this was done to decrease nulls pervasiveness in the code and increase safety
  it also means that any value that isn't an Option is guaranteed to have a value
*/

// ***** DONT DO THIS *********** there is a better way!
// to store the actual data we COULD use a struct that uses the enum type
// but there is a more concise way to do this
// struct IpAddr {
//   kind: IpAddrKind,
//   address: String,
// }
// let home = IpAddr {
//   kind: IpAddrKind::V4,
//   address: String::from("127.0.0.1"),
// };
// let loopback = IpAddr {
//   kind: IpAddrKind::V6,
//   address: String::from("::1"),
// };
// ***** DONT DO THIS ***********

fn main() {
  // to create an instances of an enum we use the name of the enum name + `::` + the variant name
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  
  
}

// having an enum like this lets each IP type be the same, aka IpAddrKind and in a function it can take:
// fn route(ip_kind: IpAddrKind) { ... }
// and then we can pass in either IpAddrKind::V4 or IpAddrKind::V6 when we call the function!
