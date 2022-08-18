pub fn run(){
    // Primitive String
    let hello = "Hello world!";
    // Growable string, heap allocated data structure.
    let mut hello2 = String::from("Hello world!");

    println!("{} , {}", hello,hello2);

    // Get Length
    println!("String length {}", hello2.len());

    // Append a character 
    hello2.push('W');
    // Append String
    hello2.push_str(", yeah!");

    // Capacity In Bytes
    println!("Capacity: {}", hello2.capacity());
    
    // Check If Its Empty
    println!("Is Empty: {}", hello2.is_empty());

    // Check if Contains
    println!("Contains: {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop through string by white space.
    for word in hello2.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');

    // Assertion Testing
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());


}