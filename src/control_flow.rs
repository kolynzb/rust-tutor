pub fn run(){
    let number = 3;

    // if statement
    if number < 5 {
    println!("condition was true");
    } else {
    println!("condition was false");
    }

    // else if
        let condition = true;
        let number = if condition {
            5
        } else {
            6
        };
        println!("The value of number is: {}", number);
        
    // Loops.
    loop {
        println!("again!");
    }

    // Conditional
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // While Loop 
    fn main() {
        let mut number = 3;
        while number != 0 {
        println!("{}!", number);
        number = number - 1;
        }
        println!("LIFTOFF!!!");
        }
    // Loop through a collection 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    // safe
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn fah_to_cel(){
    // Convert temperatures between Fahrenheit and Celsius
    
}
fn fibonacci(){
    // Generate the nth Fibonacci number
}
fn christmas(){
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas",taking advantage of the repetition in the song
}