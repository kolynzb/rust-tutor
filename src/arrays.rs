pub fn run(){
    let a = [1, 2, 3, 4];
    
    let mut numbers:[i32;5]=[1,2,3,4,5];

    println!("{:?}", numbers);

    // get  a single value
    println!("{}", numbers[0]);

    // reassign value 
    numbers[2]=20;
    // get array Length
    println!("array lenght {}", numbers.len());
    // Arrays are stack allocated 
    // Get memory taken by array in bytes
    println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));
    // Get Slice from one to three
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);
}