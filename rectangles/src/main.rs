// To make it explicit what the original two values are being used for
// a struct makes it extremely clear and can be used as a parameter for 
//the third area function; this way the function signature is more descriptive
// this also means this can be used in other functions, like displaying
// a rectangle to the screen 
#[derive(Debug)] // this lets println! macro know to print debug info from the struct
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {} square pixels.",
		area(width1, height1)
	);

	let rect1 = (30, 50);
	println!(
		"The area of the rectangle is {} square pixels.",
		area_with_dimensions(rect1)
	);

	let rect2 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The area of the rectangle is {} square pixels.",
		area_with_struct(&rect2)
	);

	/*
		note that if we wanted to see the values of rectangle while debugging the program w can try to access this with:
		println!("rect1 is {}", rect1);
		however this throws an error 
		   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
		that we need to implement the `std::fmt::Display` trait.
		the macro printLn! can do lots of formatting but it uses the above trait to do so
		this display trait is essentially the output intended for the direct end user comsumption
		the reason that this isn't defaulted is that some things may not NEED to be shown or are
		private internals that the end user doesn't need to know about; so struct by default do not
		have the display trait implemented

		However there is a neat way to circumvent this and that is to tell the println! macro we wanna Debug a value in a struct
		this section of the error explains: 
			= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
		The caveat is that each struct needs to opt into this functionality which can be done by adding a outter attribute on the struct
		#[derive(Debug)]
	*/
	// with the debug attribute opted into the struct we can now print the struct
	// it may not be the nicest but it does print all the fields and their values for the entire instance
	println!("rect2 is {:?}", rect2);
	// there is even better syntax for larger structs to output a new line for each field
	println!("rect2 is {:#?}", rect2);
	/* 
		there is another way to do this which is to use the debug macro `dbg!`
		this takes ownership of an expression versus the println! macro which takes a reference
		prints the file and line number where that dbg! happened, alongside the resultant value of that expression then
		returns ownership of that value. 
		this prints to the stderr versus println! which prints to stdout
	*/
	// example of dbg! macro in action
	let scale = 2;
	let rect3 = Rectangle {
			width: dbg!(30 * scale), // this works because it takes ownership of the expression then returns the resultant value
			height: 50,
	};
	dbg!(&rect3); // if we don't want dbg! to take ownership we can pass a reference to it
	// however because this area function is so specific to rectangles it makes sense to have it be a method of the struct

}

// this signature takes in two paramters but these are highly related and dependant on 
// each other! Instead of having them unrelated we can have them be a tuple type struct
fn area(width: u32, height: u32) -> u32 {
	width * height
}
// this way the function signature is more descriptive and we can put the data into a tuple
fn area_with_dimensions(dimensions: (u32, u32)) -> u32 {
	// notice the syntax to access each of the dimensions in the tuple
	// dot with index!
	dimensions.0 * dimensions.1
	// this is clearer than the previous function signature however it is still a little unclear
	// there isn't any indication that the first value is the width and the second is the height
	// instead we have to keep that in our mind which is which and because of that lack of
	// explaination it can easily introduce errors!
}

// this is the best way to do it because it is the most clear and descriptive it takes a reference to
// a struct type rectangle
// this also creates a immutable borrow of the struct and then ownership is maintained by the main function
// this is nice so that main can continue to use the struct after the function is called
fn area_with_struct(rectangle: &Rectangle) -> u32 {
	// note that accessing the fiels of a struct do not take ownership!
	rectangle.width * rectangle.height
}

