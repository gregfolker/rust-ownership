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
    let _y = _x;

    // A 'String' is made up of 3 parts:
    // 1) A pointer to the memory where the string is stored
    // 2) A length (How much memory, in bytes, the string is currently using)
    // 3) A capacity (The maximum amount of memory, in bytes, the string can use)
    //
    // The above three fields are stored on the stack whereas the
    // string itself is stored in the heap
    let s1 = String::from("Hello");

    // Copy the contents of s1 into s2 (The pointer, the length, and the capacity)
    // Doing so invalidates s1 to avoid a double free error when the current
    // scope ends
    // This is known as a 'shallow copy', which Rust does by default for runtime performance
    let s2 = s1;

    // The following is a compiler error because we are trying to access
    // a variable that Rust considers to no longer be valid
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    // In order to perform a 'deep copy', e.g., copy the contents from the stack
    // AND the heap, the 'clone' method must be used
    let s3 = String::from("Hello");
    let s4 = s3.clone();

    // Now s3 is still considered to be valid and it is no longer a compiler
    // error to reference s3
    println!("s3 = {}, s4 = {}", s3, s4);

    // 'Copy' can only be used for variables that reside on the program stack,
    // e.g.,  what is happening on Lines 27-28 for '_x' and '_y'

    // The same rules apply when working with functions outside of main
    // `drop` is called automatically by Rust when the function scope ends
    let s = String::from("Hello");

    // `s` is no longer valid here (Out of scope) because it was moved
    // into the function `takes_ownership()`
    takes_ownership(s);

    let x = 5;

    // `x` moves into the function but since this is just a simple `Copy`,
    // x remains in scope here. So, it is perfectly valid to reference `x`
    // after this line
    makes_copy(x);

    println!("{}", x);

    // Ownership is returned to the caller, so it
    // is valid for us to use `z` after this point
    let z = gives_ownership();

    println!("{}", z);

    // We have to reallocate `s` in this scope because
    // we gave ownership with the call to `takes_ownership(s)` above
    let s = String::from("Hello");

    println!("{}", takes_and_gives_back(s));

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len)

} // End of the 'main' scope

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    // Return some_string, giving ownership to the caller of this function
    some_string
}

fn takes_and_gives_back(s: String) -> String {
    // Here, `s` is returned back to the original scope
    // where this function was called
    s
}

// Similar to Go, you can return multiple values from functions
// in Rust as a tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    // `s` has to be returned here so the caller
    // can regain ownership of the memory allocated for `s`
    (s, length)
}
