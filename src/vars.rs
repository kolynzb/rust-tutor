pub fn run(){
    let name= "Kolynz";
    // Mutable Variable
    let mut age = 37;
    println!("hello am {} and i am  {}", name,age);

    // its my birthday so print
    age = 38;

    println!("hello am {} and i am now  {}", name,age);

    // Define a constant.
    const ID:i32 =001;
    println!("Constant variable ID: {}",ID);
    
    // Assign multiple variables
    let (school,year)=("kyu",1);
    println!("am at {} in year {}",school,year);


}