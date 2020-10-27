// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Primitive str
    let hello_prim = "Hello";

    // String
    let mut hello_string = String::from("Hello ");

    // Get Length
    println!("Length: {}", hello_prim.len());

    // Push/add on a char
    hello_string.push('W');

    //Push/add a string
    hello_string.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello_string.capacity());

    // Is empty
    println!("Is Empty: {}", hello_string.is_empty());

    // Contains
    println!("Contains 'World': {}", hello_string.contains("World"));

    // Replace
    println!("Replace: {}", hello_string.replace("World", "There"));

    // Look through string by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello_prim, hello_string));
}