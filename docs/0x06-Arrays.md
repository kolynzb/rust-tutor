# Arrays in Rust

- Arrays in rust have a fixed number of elements of the same datatype.

`syntax`

```rs
// initialization
 let array_name:[datatype;length]=[element1,element2];

// Example
 let numbers:[i32;5]=[1,2,3,4,5];

//  To  print we use the debug trait
 println!("{:?}", numbers);

```

- To get a single value we can use square braket notation with the index of the element.
  ` println!("{}", numbers[0]);`

- To change an element make sure the array is mutatable
  ` let mut numbers:[i32;5]=[1,2,3,4,5];`
- Reassign values to arrays
  `numbers[2]=20;`
- Get Array Length
  `println!("array lenght {}", numbers.len())`
- **Note that** Arrays are stack allocated.
- To get memory taken by array in bytes
  ```rs
  println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));
  ```
- Get Slice from one to three
<!-- TODO: WHAT IS THE AND PRESANT FOR -->

```rs
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}",slice);
```
