# Datatypes

- Rust is a _statically typed language_ and it must know the types of all variables at compile time,however,the compiler can usually infer what type we want to use based on the value and how use it.

- **Scalar types** - Represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
- **Integer types** - is a number without a fractional component.This type declaration indicates that the value it’s associated with should be an unsigned integer (signed

<!-- ## Primitive types -->

```rs
  let x=1;
  let z:i64 = 453434343434;
```

- Integers: u8,i8,i16,u32,i32,u64,i64,u128,i128 (_number of bits taken in memory. Where u is `unsigned integer` meaning it only takes postive values.The larger the number, the larger the bits allocated_).

  | Length  | Signed | Unsigned |
  | ------- | ------ | -------- |
  | 8-bit   | i8     | u8       |
  | 16-bit  | i16    | u16      |
  | 32-bit  | i32    | u32      |
  | 64-bit  | i64    | u64      |
  | 128-bit | i128   | u128     |
  | arch    | isize  | usize    |

- Each signed variant can store numbers from −(2^(n − 1)) to (2^(n − 1)) − 1 inclusive, where n is the number of bits that variant uses.

### So when do i use i64 or i32

- if your integer is less than 2147483647 then use `32` where as if it is less than 9223372036854775807 use `64` .
- To get the max values for these types.We can use rust's standerd librard `std`.

```rs

 println!("Max i32 {}",std::i32::MAX);
 println!("Max i64 {}",std::i64::MAX);

```

- If you’re unsure, Rust’s defaults are generally good choices , and integer types default to i32: this type is generally the fastest, even on 64-bit system.When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term _panicking_ when a program exits with an error.When you’re compiling in release mode with the --release flag, Rust doesnot include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, 256 becomes 0, 257 becomes 1,and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have.Relying on integer overflow’s wrapping behavior is considered an error. If you want to wrap explicitly, you can use the standard library type Wrapping.

## Floating point Types

- Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

```rs
fn main() {
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
}
```

- **Integer Overflow** - Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur.

## boolean values

- inferred boolean.
  `let is_active = true;`
- typed boolean.
  `let is_tall:bool = true;`
- Boolean from an expression.
  ` let is_greater = 10>2;`

## Boolean (bool)

- This have a size of 1bit

```rs
 let is_active = true;
 let is_tall:bool = true; // explicitly typed
```

## Character

- char is a unicode character.This can be any unicode value.(_most primitive alphabetic type and is 4bits_)
- We use single quotes for this one as oppossed to string literals that use double quotes.

```rs
let a1 = 'a';
let happy = '😥';
// using emoji unicodes
let emoji:char = '\u{1F600}';
```

- To get unicode characters [Click Me 😋](https://www.toptal.com/designers/htmlarrows/punctuation/middle-dot/)

# Coumpound Types

- These can group multiple values into one type. Rust has two
  primitive compound types: tuples and arrays

## Tuples

- A tuple is a general way of grouping together some number of other values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size._For more [click me 😉](./0x05-Tuples.md)_

## Arrays

- Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.[Click For more info 🔥](/docs/0x06-Arrays.md).
