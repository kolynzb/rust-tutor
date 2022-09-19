# Must knows

- Functions are declared with `fn` keyword and the main program must have a main fuction.

```rs
fn main(){
    // code goes here
}
```

- `pub` makes the function public.
- `mod` imports a funtion from one file to another.
- Print statements in rust are written using a `println!("hello ")` macro(_function_).

`main.rs`

```rs
mod print;

fn main() {
    // to call the function in print
    print::run();

}
```

# Basic Print Formatting In Rust.

- To print an integer or a variable you would have to use place holders.
  `println!("{}",2)`
- To use multiple placeholders to print
  `println!("{} is {} ","Collins","Awesome");`
- If you want to repeat a single variable multiple time in the string you can use _positional arguements_.for instance if i wanted to print "Kolynz is learning rust and Kolynz likes rust"
  `println!("{0} is learning {1} and {0} likes {1}","kolynz","rust",);`
- Or you can use named arguements.(_john likes mangoes_)
  ` println!("{name} likes to eat {food}",name = "john",food = "Mangoes")`
- _Placehoder traits_ - help when you want to out put something in a particular format for instance out put 10 in binary , hexadecimal and octal.(_Binary: 1010 Hex:a Octal:12_).
  `println!("Binary: {:b} Hex:{:x} Octal:{:o}",10,10,10);`

- Perform basic Arithmetic operations.
  `println!("10 + 10 = {}",10 + 10);`
- Placeholder for debug traits.
  `println!("{:?}",(12,true,"hello")); `

- To get user input use import std `use std::io` and
  `let mut name=String::new();`
