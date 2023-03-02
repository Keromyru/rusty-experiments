// for example a User struct has the keyword, name to describe the grouping of data and then named and typed fields
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main(){
  /*
    structs or structures, is a custom data type that lets different name and values to be grouped
    and packaged together, making a meaningful unit/grouping. Structs are similar to object's attributes and
    similar to tuples but have lots of differences and either or could be good for different situations.
    this also includes struct functions aka methods! :D
  */
  /*
    Obviously tuples do not have named data versus structs, similar to objects, do have named data/methods
    Tuples have ordered data versus structs do not
  */

  // to instantiate a struct, we use the struct name and then curly braces with the fields and values
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  // we can make the instance mutable by adding the mut keyword
  let mut user2 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };
  // re assign the value of the email field!
  user2.email = String::from("anotheremail@example.com");
  // ** note: rust doesn't allow for only certain fields to be mutable, it is all or nothing **

  // there is a builder function further down that handles building a user struct
  // this lets us create a new User instance regularly without updating and syntax
  let user3 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
  };
  // there is also struct update syntax to make this even easier
  // the ..<struct_instance> specifies that the remaining fields not explicitly set should have the 
  // same value as the fields in the given instance; note this has to come after the fields that are set
  // with different values for it to function correctly
  let user4 = User {
    email: String::from("another@example.com"),
    ..user1
  };

  /*
    VERY IMPORTANT this is like variables and moving data, the first user1 instance is moved to user4
    (though this file will likely throw a bunch of errors because of all the users using user1)
    so user1 can no longer be used because the data has moved. 
    the String in the username field of user1 was moved into user2. If we had given user2 new String 
    values for both email and username, and thus only used the active and sign_in_count values from user1, 
    then user1 would still be valid after creating user2. Both active and sign_in_count are types that 
    implement the Copy trait, so the behavior we discussed in the section would apply.
  */

  /*
    there are also tuple structs that have extra meaning WITHOUT the named fields.
    this is useful if you want give the whole tuple a name and make the tuple a different 
    type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
  */

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  /*
    there is another type of struct called a unit-like struct that doesn't have any fields.
    They can be useful when we have to implment a trait on some type but don't have any data that
    needs to be stored in the type itself. More on this later but an example:
  */
  struct AlwaysEqual;
  let subject = AlwaysEqual;

  
}

// returns the new user struct!
// it makes sense naming the parameters the same as the struct but it can be tedious
// instead this could use the field shortcut
fn build_user(email: String, username: String) -> User {
  User {
      active: true,
      username: username, // if the same name can just do username
      email: email, // if the same name can just do email
      sign_in_count: 1,
  }
}