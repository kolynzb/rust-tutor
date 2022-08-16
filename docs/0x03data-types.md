## Primitive types

- Integers: u8,i8,i16,u32,i32,u64,i64,u128,i128 (_number of bits taken in memory. Where u is `unsigned integer` meaning it only takes postive values.The larger the number, the larger the bits allocated_).

```rs
  let x=1;
  let z:i64 = 453434343434;
```

- Floats: f32,f64
  `let y =2.5;`
- Boolean (bool)

```rs
 let is_active = true;
 let is_tall:bool = true;
```

- Charaters (char)
- Tuples
- Arrays

- Rust is a _statically typed language_ and it must know the types of all variables at compile time,however,the compiler can usually infer what type we want to use based on the value and how use it.

### So when do i use i64 or i32

- if your integer is less than 2147483647 then use `32` where as if it is less than 9223372036854775807 use `64` .
- To get the max values for these types.We can use rust's standerd librard `std`.

```rs

 println!("Max i32 {}",std::i32::MAX);
 println!("Max i64 {}",std::i64::MAX);

```

## boolean values

- inferred boolean.
  `let is_active = true;`
- typed boolean.
  `let is_tall:bool = true;`
- Boolean from an expression.
  ` let is_greater = 10>2;`

## Character

- char is a unicode charactar.This can be any unicode value.
- We use single quotes for this one.

```rs
let a1 = 'a';
// using emoji unicodes
let emoji:char = '\u{1F600}';
```
