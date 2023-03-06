/*
  A crate can come in one of two forms: a binary crate or a library crate. 
  Binary crates are programs you can compile to an executable that you can run, 
  such as a command-line program or a server. Each must have a function called 
  main that defines what happens when the executable runs. All the crates we’ve 
  created so far have been binary crates.

  Library crates don’t have a main function, and they don’t compile to an executable. 
  Instead, they define functionality intended to be shared with multiple projects. 
  For example, the rand crate we used in Chapter 2 provides functionality that generates 
  random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, 
  and they use “crate” interchangeably with the general programming concept of a “library".

  There’s also a src directory that contains main.rs. Open Cargo.toml in your text editor, 
  nd note there’s no mention of src/main.rs. Cargo follows a convention that src/main.rs 
  is the crate root of a binary crate with the same name as the package. Likewise, Cargo
  knows that if the package directory contains src/lib.rs, the package contains a library 
  crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes 
  the crate root files to rustc to build the library or binary.

   If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library,
    both with the same name as the package. A package can have multiple binary crates by placing 
    files in the src/bin directory: each file will be a separate binary crate.
*/

fn main() {
    println!("Hello, world!");
}
