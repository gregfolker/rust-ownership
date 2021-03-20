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

    // The following binds the value '5' to x, and
    // then makes a copy of x in y
    let _x = 5;
    let _y = x;

    // A 'String' is made up of 3 parts:
    // 1) A pointer to the memory where the string is stored
    // 2) A length (How much memory, in bytes, the string is currently using)
    // 3) A capacity (The maximum amount of memory, in bytes, the string can use)
    //
    // The above three fields are stored on the stack whereas the
    // string itself is stored in the heap
    let s1 = String::from("Hello");
    let s2 = s1;


} // End of the scope
