# Variables In Rust.

- _Real Quick though_. In rust, variables are immutable by default which is not the case in other programming languages.(_when defined they cannot be reassigned a new value_)
- They hold primitive data or references to data.
- Rust is a block-scoped language.
- we use `let` keyword to initialize a variable.
  `let name= "Kolynz";`
- To make a variable mutable you add `mut` key word.

## Define a constant.

- We use the `const` keyword.
- you must explicitly define a type and the name must be uppercase.
  `const ID:i32 =001;`
- This is simply a constant called "ID" with a type of interger 32bit.

## Assigning multiple varibles.

- _Multiple variable assignment_

```rs
 let (school,year)=("kyu",1);
 println!("am at {} in year {}",school,year);

```

