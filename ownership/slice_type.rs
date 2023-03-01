fn main(){
  // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
  // A slice is a kind of reference, so it does not have ownership.

  // explination and function example below!
  
  // however there is a problem using the first word function that return usize; take this example: 
  let mut s = String::from("hello world");
  let word = first_word(&s); // word will get the value 5
  s.clear(); // this empties the String, making it equal to ""
  /* word still has the value 5 here, but there's no more string that
    we could meaningfully use the value 5 with. word is now totally invalid!
  */
  /* More concerningly this would compile because word isn't meaninfully connnected
    so the compiler cannot guard against this; it also makes it very brittle because
    we have to keep track of it in weird ways.

    This gets even worse with a second word function that would look like:
          fn second_word(s: &String) -> (usize, usize){
    There is would be three chunks of related but not connected data that would have to be tracked
    this is super brittle and error prone
    to solve this use slices!
  */
  // ** String slice **
  let s2 = String::from("hello world");
  // the slice syntax is [starting_index..ending_index] and is always a reference
  // this is a slice of the string s that starts at the first byte and ends at the 5th byte
  // this references the string S it came from, so if that changes this should too
  let hello = &s2[0..5];
  // this is a slice of the string s that starts at the 6th byte and ends at the 11th byte
  let world = &s2[6..11];
  /*
    When there is a slice there is a pointer to the byte specified at the starting index and a length
    that indicates how many bytes it ends at.
  */
  // you can drop the starting number if you want it to start at the beginning
  let s3 = String::from("hello");
  let slice_s3_1 = &s3[0..2];
  // this the same as saying 0 to index 2
  let slice_s3_2 = &s3[..2];
  // the inverse is true as well, if you want the end of a string can drop the last number
  let s4 = String::from("hello");
  let len = s4.len();
  let slice_s4_1 = &s4[3..len];
  let slice_s4_2 = &s4[3..];
  // you can also drop both numbers for the whole string! Which is weird but okay
  let slice_s4_3 = &s4[..];
  // ** Note: this assume ASCII characters because multibyte characters will break if trying to the middle of it ** 
  // more on this later

  // rewriting first_word to use a slice!\
  // see below

  // this can laos work for the second word function as well, instead of returning two byte indexes
  // this instead can also return on string slice. This is much more flexible and less brittle/error prone

  /*
    this API it much better because it is harder to make mistakes as it needs to ensure it is valid for the 
    underlying string; just by using a slice it is impossible to get the error from before.
    This is because clear requires a mutable reference and because the slice is borrowing a immutable reference
    these things cannot exist at the same time when the immutable reference is used in a print statement but the 
    string is cleared aka mutable/mutated.
  */
  /*
    This is important knowledge as it pertains to string literals. String literals are stored in the binary.
		the variables that refer to a string literal are immutable. this can also improve the function signature
		of first word to use &str as the param, this would work with both &String and &str this takes advantage
		of the deref coercions but that will be discussed later.
  */
	/*
		Slices can be used for other types as well, such as arrays. It makes the same way as strings
		the slice is a reference with a pointer to the starting index and a length to indicate where
		that reference ends. This can be done on all sorts of collections!
	*/
	let a = [1, 2, 3, 4, 5];
	let slice_a = &a[1..3];
	assert_eq!(slice_a, &[2, 3]);


}

/*
  the return type for a string slice is noted as &str
*/
fn first_word_with_slice(s: &String) -> &str {
  let bytes = s.as_bytes();
  // the same process for finding the first work in the previous function
  // but this is tied to the underlying data of the original string
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      // though instead of returning the byte index, we return the slice of the string
      // notice i being used to specify the end of the slice dynamically
        return &s[0..i];
    }
  }
  // otherwise return the whole string
  &s[..]
}

/*
  Programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
  If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
*/
// this function has a references string since we don't want ownership of the string
// and instead of returning part of a string, we return a number that represents the index of the end of the first word
fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes(); // converts the string into an array of bytes so we can loop for each element in the byte array

  /*
    iter is creating a interator for the array of bytes; enumerate wraps the result of iter and 
    returns each element as a part of a tuple instead. this is a much more convient way to get the index
    without manual calculations.

    because it is a tuple we can destruct the tuple into two variables, specified by the pattern (i, &item)
    more on patterns later, but importantly because the enumerate gives a reference we have to use &item 
  */
  for (i, &item) in bytes.iter().enumerate() {
    // this is checking, using a byte literal, if the current byte is a space
    // if it is found the index is returned
      if item == b' ' {
          return i;
      }
  }
  // otherwise the length of the string is returned as it is the first word
  s.len()
}