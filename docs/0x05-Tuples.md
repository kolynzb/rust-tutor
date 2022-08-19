# Tuples In Rust

- Tuples group together values of different types.
- Max 12 elements.
  `let person:(&str,&str,i8)=("Brad","Collins",37);`
- When declaring tuples indicate there types in the parenthesis after the semicolon and name.For instance,In the above example the tuple has its first two elements as string literals and the last an 8 bit integer.

- To reference the elements in the tuple, use dot notation.
  ` println!("{} is from {} and is {}",person.0,person.1,person.2);`
