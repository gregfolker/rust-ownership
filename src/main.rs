// Project: ownership
// Author: Greg Folker

fn main() {
	println!("Hello, World!");

    // The variable 's' is a string literal
    // This variable is valid from the point it is declared until
    // the end of it's current scope
    let _s = "Hello";

    // Rust has another type of string, 'String'
    // This type is allocated on the heap and the size is therefor dynamic
    // You can create a 'String' from a string literal using 'from'
    let _s = String::from("Hello");

    // This kind of string can be mutable
    let mut hello_world = String::from("Hello");

    // push_str() appends a literal value to a string
    hello_world.push_str(", World!");

    println!("{}", hello_world);

} // End of the scope
