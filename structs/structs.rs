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
  // for example a User struct has the keyword, name to describe the grouping of data and then named and typed fields
  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }
  // to instantiate a struct, we use the struct name and then curly braces with the fields and values
  let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

  
}