# Strings

- _Primitive str_ - Immutable fixed-length string somewhere in memory.
  `let hello = "hello";`
- _String_ - Growable , heap-allocated data structure
- Use when you need to modify or own string data.
  `let hello = String::from("hello");`

  - Get Length
    `println!("String length {}", hello2.len());`

  - Append a character
    `hello2.push('W');`
  - Append String
    `hello2.push_str(", yeah!");`

  - Capacity In Bytes
    `println!("Capacity: {}", hello2.capacity());`

  - Check If Its Empty
    ` println!("Is Empty: {}", hello2.is_empty());`

  - Check if Contains
    `println!("Contains: {}", hello2.contains("World"));`

  - Replace
    `println!("Replace: {}", hello2.replace("World", "There"));`

  - Loop through string by splitting with white space.

  ```rs
  for word in hello2.split_whitespace(){
   println!("{}", word);
   }
  ```

  - Create string with a specified capacity

  ```rs
   let mut s = String::with_capacity(10);
   s.push('a');
  ```

- Assertion Testing

  ```rs
  assert_eq!(2,s.len());
  assert_eq!(10,s.capacity());
  ```
