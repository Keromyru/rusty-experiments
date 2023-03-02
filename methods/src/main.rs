#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// to define a function within the context of a struct we use the impl keyword
// impl is short for implementation
impl Rectangle { // the functions here are associated functions, because they are associated 
  // with the Rectangle struct (or another struct that uses impl)

  /*
    we can also define associated functions that don't have self as their first parameter
    (which makes them not methods) because they don't need an instance of the type to work with
    example: String::from function that is defined on the string type

    These types of functiosn are often used for constructors that will return a new instance of the struct
    these are often called new but new isn't a special name and isn't built into the language
    We could provide an associated function square that takes a single parameter and uses that value
    to create a square rectangle and returns Self aka the instance of the struct
  */
  // the self keyword in the return type and body are aliases for the type that appears after the impl keyword
  fn square(size: u32) -> Self {
    Self { width: size, height: size }
  }
  /*
    to use this function we would call Rectangle::square(3) which would return a Rectangle instance
    the `::` syntax is used to call associated functions on structs as well as namespaces in modules
  */

  // the area function gets changed to &self to indicate that it is a method
  // and it is looking into its own internal structure to get the width and height
  // and then use the method syntax to call the method.
  fn area(&self) -> u32 { // note &self is shorthand for self: &Self
      self.width * self.height
  }

  // when using a function like this Rust understands that calling rect1.width() means the method and
  // rect1.width is the field 
  fn width(&self) -> bool{
    self.width > 0
  }

  // Note that the first parameter, self, is in the signature but not in the body of the method
  // it also doesn't need to be added when called later
  // the signature is an immutable borrow of a Rectangle instance
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  /*
    each struct can have multiple impl blocks but there is no reason not to have them all in one impl block
    Later there is a reason to have multiple impl blocks which is generic types and traits
  */
}
/*
  Methods must have a parameter called self as their first parameter
  methods can use references to self to borrow the Self instance (&self)
  but methods can take ownership of self, borrow self immutability OR mutably (&mut self)(?) 

  in the area case, we use &self because we don't want to take ownership of the instance just read data.
  if we needed to change the instance that we've called a amethod on as part of what that method does
  we'd use &mut self as the first parameter.
  Having a method take full owernship of the instance by using just self as the first parameter is rare;
  this technique is usually used when the method transforms self into something else and you want to prevent
  the caller from using the original instance after the transformation.

  Methods can also have the same name as a attribute; like width method

  ** Note: Getters aren't created by default to create private fields but public read only getters, more on this later **
*/

/*
  Rust has a feature called automatic refercing and dereferencing calling a method with object.something() rust
  automatically adds in &, &mut, or * so object matches the signature of the method. 
  this works for methods because they have a clear receiver, the type of self. given the receiver and the name of the method
  Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self) the receiver.
  this makes ownership ergonomic in practice! 
*/

fn main() {

  /* 
    methods are functions that are defined within the context of a struct, enum, or trait object
    they are still declared with `fn`, they can have params and return values
    the first param is always self, wich is the instance of the struct the method is being called on.
  */


  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
  }

  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };
  
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
