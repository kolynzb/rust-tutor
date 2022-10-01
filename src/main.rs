mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod struts;

mod guessing_game;
fn main() {
    // 01-PRINT AND FORMATTING.
    print::run();
    // 02-VARIABLES.
    vars::run();
    // 03-DATATYPES
    types::run();
    // 04-STRINGS 
    strings::run();
    // 05-Tuples
    tuples::run();
    // 06-Arrays
    arrays::run();
    // 07-Vectors
    vectors::run();
    
    // Rust Book
    
    // 02-Guessing Games.
    guessing_game::run();
    // 03-Guessing Games.
    struts::run();
    
}
