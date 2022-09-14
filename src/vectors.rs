use std::mem;

pub fn run(){
    let mut numbers:Vec<i32>= vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // get  a single value
    println!("{}", numbers[0]);

    // reassign value 
    numbers[2]=20;

    // Add onto Vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    // get  Length
    println!("array lenght {}", numbers.len());
    
    // Get memory taken by Vecctor in bytes
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));
    // Get Slice from one to three
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);

    // Loop Through
}