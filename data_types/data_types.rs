use std::io;

fn main(){
  // there is inferred types but it is a statically typed language so this:
  let guess: u32 = "42".parse().expect("Not a number!"); // without u32
  // fails because the compiler doesn't know what value we want
  
  // there are four Scalar (singular value) types in Rust
  // integers, floating-points, boolean and characters


  // ** integers **
  // i and u are signed integers and unsigned respectively, so u32 is unsigned aka cannot have negative numbers
  // stored with twos complements  : https://en.wikipedia.org/wiki/Two%27s_complement

  // Length |	Signed | Unsigned
  // --------------------------
  // 8-bit  |   i8   | u8
  // 16-bit	|   i16	 | u16
  // 32-bit	|   i32	 | u32
  // 64-bit	|   i64	 | u64
  // 128-bit|	  i128 | u128
  // arch	  |  isize | usize

  // arch is dependant on computer architecture 
  
  // number literals can be in a special way to make them easier to read!
  // includin underscores!
  // Number literals |	Example
  // ---------------------------
  //   Decimal	     |  98_222
  //   Hex	         |  0xff
  //   Octal	       |  0o77
  //   Binary	       |  0b1111_0000
  //  Byte (u8 only) |  b'A' 

  // if integer overflow happens, the debug mode will include checks for it
  // otherwise the program will panic!
  // if it is compiled for release mode, it instead will be perform two's complement wrapping
  // and the value goes back to its lowest value it can (aka wraps over)
  // but this is considered erroneous
  // there are standard methods to handle if overflow can happen! 


  // ** floating point **
  // always signed, only f32 and f64 which most operating systems are the latter
  
  //  let x = 2.0; // f64
  // let y: f32 = 3.0; // f32

  // Rust supports the basic mathematical operations you’d expect for all the number types: 
  // addition, subtraction, multiplication, division, and remainder. 
  // Integer division truncates toward zero to the nearest integer

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;


  // ** boolean **
  let t = true;
  let f: bool = false; // with explicit type annotation


  // ** character **
  // char s are always single quote '' and strings are always double quotes ""
  let c = 'z';
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';
  // and they can represent way more than ASCII 
  // Accented letters; Chinese, Japanese, and Korean characters; emoji; and 
  // zero-width spaces are all valid char values in Rust. Unicode Scalar Values 
  // range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
  // however personal understanding of character may not match what is it in rust


  // ** compound type **
  // compound types are a way of grouping multiple values into one type. there 
  // are two primitive compound types, arrays and tuples
  
  // * tuples *
  // tuples cannot grow in size after declared and each position needs to be typed
  // tho it can be a mix of multiple types 
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // to access the values in a tuple can use pattern matching to destructure the tuple value:
  let (x, y, z) = tup;
  println!("The value of y is: {y}");
  // you can also access each value in a tuple with dot notation and an index to access
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
  // a tuple without values is called a unit which is a special value and represents an empty
  // value or empty return type; expressions implicitly return the unit value if they don't return anything else

  // * arrays *
  // a collection of multiple values of the same type and are fixed length
  let a = [1, 2, 3, 4, 5]; 
  // arrays are on the stack and not the heap (vectors can grow in size)
  // arrays are useful for when you know it won't grow in size, like the months
  let months = ["January", "February", "March", "April", "May", "June", "July",
  "August", "September", "October", "November", "December"];
  // to type an array it needs a square bracket with the type and number of elements in the array
  let b: [i32; 5] = [1, 2, 3, 4, 5];
  // arrays can also have the same value for each element by specifying the initial value
  // followed with a semicolon and then the length of the array
  let c = [3; 5];
  // arrays are a single chunk of memory of a known, fixed size can be allocated to the stack
  // it can be accessed with array indexing
  let first = a[0];
  let second = a[1];


  // in this example the array can be accessed outside the length of the array and 
  // result in a runtime error 
  // let a = [1, 2, 3, 4, 5]; // already defined above 

  println!("Please enter an array index.");

  let mut index = String::new();

  io::stdin()
      .read_line(&mut index)
      .expect("Failed to read line");

  let index: usize = index
      .trim()
      .parse()
      .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");
  // interestingly unlike other low level langauges the memory
  // outside the bounds of the array will be accessed and result in 
  // reading data it shouldn't. And there is no way for the complier to
  // know what the user is going to input. 
  // but it always checks the index before accessing and stops this 
  // it is its memory safety principles in action!
}