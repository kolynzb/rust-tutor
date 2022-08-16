pub fn run (){
    // Default is "32"
    let x=1;
    // Default is "f64"
    let y =2.5;
    // Add explicit type
    let z:i64 = 453434343434;

    println!("Max i32 {}",std::i32::MAX);
    println!("Max i64 {}",std::i64::MAX);
    // Boolean
    let is_active = true;
    //Boolean from expression
    let is_greater = 10>2;
// Character
let a1 = 'a';
let emoji:char = '\u{1F600}';
    println!("{:?}",(x,y,z,is_active,is_greater,a1,emoji));
}