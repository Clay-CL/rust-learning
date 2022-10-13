## Variables
- `let` by default is like `val` in kotlin.
- with `mut` it's like var.
- `const` values must have types declared

- it looks dynamically typed, but in fact it is statically typed.
```rust
// you could do this
let spaces = " ";
let spaces = spaces.len();
```

- but if you try to use `mut` here, you'll get a compile time error
```rust
// you cannot do this
let mut spaces = " "; // causes a compile time error
let spaces = spaces.len();
```
- mut ensures the sanctity of the type for the variable `spaces`

## Scalar types
- represents a single value in rust
- integers, floating-point, boolean, characters
### Integers
- signed types are **i<bits>**
- unsigned are **u<bits>**
- Examples:-
     1. 8bit signed **i8**
     2. 32 unsigned **u32**
- some integer literals
  - 1000 is the same as 1_000
  - Hex - 0xff
  - Byte (`u8`) - `b'A'`
  - Binary `0b1111_0000`

### Floating points
`f32` and `f64` are floating point types

> Note : Integer/ Integer division is floored

### Boolean
type is represented as `bool`

### Character
type is represented as `char` 
- All utf-8 encoded chars

## Compound types
### Tuple
- grouping of variety of types in one compound one
- fixed length, once declared they cannot grow/shrink
```rust
let tuple_ex: (i32, f64, u8, char) = (2, 7.8, 1, 'J');
```
- destructing is supported; like in typescript and kotlin
```rust
let (x,y,z,w) = tuple_ex;
```
- individual elements can be accessed via index
```rust
let x = tuple_ex.0;
let w = tuple_ex.3;
```
- tuple without any values is called **unit**

### Array
- fixed type and fixed length
- Useful when data needs to be allocated on the stack rather than heap
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

// to initialize an array
// a contains 5 elements of 3
let a = [3; 5];

// a would look like
[3, 3, 3, 3, 3]
```
- can be indexed normally
- For index_out_of_bounds, a panic would occur

## Functions
- syntax is `fn` followed by name
```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add_2_ints(a: i32, b: i32) -> i32 {
    a + b
}

let x = add_2_ints(3, 4);

// this is the same as
// let x = a + b;
// a = 3, b = 4;
```
- hence you don't need the return statement

## Control Flow
### If statements
```rust
if num < 5 {
    // DO something
} else {
    // DO something else
}
```
- if expressions are sometimes referred to as arms

#### Using if with let
- In rust you can do something like you do in kotlin, use if, when to assign variables
```rust
let num = if x < 5 { 100 } else { -1 };
```
- you cannot assign different types in this case
- For instance, you cannot do this:
```rust
let number = if condition { 5 } else { "six" };
```

## Loops
- 3 different loops - *loop, while and for*
### loop
- executes a block of code infinitely 
- you could return a value from a loop, when broken
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

- you can have labels to distinguish between loops
- break and continue can be used with labels
```rust
fn main(){
	let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                brek 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
``` 
