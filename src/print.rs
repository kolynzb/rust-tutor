pub fn run(){
    // printing hello world
    println!("hello");

    // use placeholders to print numbers or variables
    println!("{}",2);
    // using multiple placeholders to print
    println!("{} is  {} ","Collins","Awesome");

    // Positional Arguements 
    println!("{0} is learning {1} and {0} likes {1}","kolynz","rust",);

    // Named arguements
    println!("{name} likes to eat {food}",name = "john",food = "Mangoes");

    // Placeholder traits

    println!("Binary: {:b} Hex:{:x} Octal:{:o}",10,10,10);

    // Placeholder for debug trait (debug placeholder).
    println!("{:?}",(12,true,"hello"));

    // basic Arithmetic
    println!("10 + 10 = {}",10 + 10);

}