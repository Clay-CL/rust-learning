## Memory in Rust
> Also referred to Ownership. A set of rules governing how Rust manages memory
- **Compile Time** check of memory management. Very memory safe.
- Stack
- Heap

### Ownership Rules
- Each value in Rust has an owner
- Single owner at any given time
- When owner out of scope, value is dropped.

### Scope of a Variable
```rust
{   // s invalid here
    let s = "hello"; // from here to the end of the scope, s is valid
    
} // s becomes invalid from here
```

### Strings
- String is a complex type in Rust. It is of variable length.
- They reside in the heap

```rust
let s = String::from("hello");

s.push_str(", world!");
println("{}", s);
```
- How is that a `String` can be mutated but not a literal?
- In case of a literal, size is determined at compile time
- The aim is to ensure that memory is provided when requested upon and
- The memory is returned to the allocator when finished.
- This **allocate-free** pair of memory usage for variables is what empowers Rust, 
  which does is using the scope.

- When a variable goes out of scope, a special function called `drop` is called.
- Similar to RAII(Resource Acquisition Is Initialization) in C++


### Interaction of Variables and Data
```rust
let x = 5;
let y = x;
```
- In the above snippet, a value 5 is bound to x
- a copy of 5 is bound to y, making x and y both equal to 5, but in different addresses
- This can happen since the size of 5 is known and they are pushed to the stack.

```rust
let s1 = String::from("Hello");
let s2 = s1;
```
> when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable.
- when this is done to variables stored in the heap, Rust only moves the data and doesn't automatically creates deep copies of them

### Ownership and functions
- if a variable is passed in a function as a parameter, it loses ownership once it is completed being used by the function
```rust
fn main() {
    ....
    let s = String::from("hello");

    some_function(s);
    
    println("s = {s}"); // this would throw an error when compiled
    ....    
}

fn some_function(some_string: String) {
    println!("{some_string}");
} // after this line, the variable some_string goes out of scope
// the drop function is called then.

```
- Rust does let us return multiple values using a tuple
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

## References and Borrowing
- The issue with the above code is that to use the string again after calculating it's length;
 One would need to return it
- the above code can be optimized as follows:
```rust
fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
}

fn calculate_length(s: &String) -> usize {
  s.len()
} // After this s goes out of scope, but since it doesn't have ownership
// of what it is referencing, it is not dropped 
```
Just like in cpp, in rust too
- `&` -> referencing
- `*` -> dereferencing

> This action of creating a reference is called `Borrowing`

- One cannot modify something that is being borrowed, just like a rented bike

```rust
fn change_str(some_string: &String) {
  // this will throw an error during compile time
  // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
  some_string.push_str(", World!");
}
```

### Mutable References
- the above code snippet can be fixed by declared the parameter as mutable
- and by passing a mutable reference
```rust
fn main() {
  let mut s = String::from("hello");
  change_str(&mut s);
}

fn change_str(some_string: &mut String) {
  some_string.push_str(", world!");
}
```
- there is a restriction though : you can only borrow a mutable reference once at a time.
- the following code will fail compilation
```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // will cause error here
// error[E0499]: cannot borrow `s` as mutable more than once at a time

println("{}, {}", r1, r2);
```
- the only borrow once at a given time is for mutation but in a controlled fashion
- This prevents data races. A data race is when
   - 2 or more pointers access the same data at the same time
   - At least one of the pointers is used to write the data
   - No synchronization
- Rust doesn't compile code with data races
- The error code above can be altered to ensure both r1 and r2 can mutate s
- You could 
  1. Scope it out
      ```rust
      let mut s = String::from("Hello");
      {
        let r1 = &mut s;
      }
      let r2 = &mut s;
      ```
  2. Borrow it after r1 is out of scope
      ```rust
      let mut s = String::from("hello");
      let r1 = &mut s;
      println!("r1 : {}",r1);

      let r2 = &mut s;
      println!("r2 : {}",r2);
      ```
  1. Let r2 borrow a mutable r1
      ```rust
      let mut s = String::from("hello");
      let mut r1 = &mut s;
      let r2 = &mut r1;
      println!("{}, {}", r1, r2);
      ```
- There is also a rule for combining mutable and immutable references
This code is erroneous
```rust
let mut s = String::from("Hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // PROBLEM
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

println!("{}, {}, {}", r1, r2, r3);
```

### Dangling References
- Essentially a pointer pointing to an invalid memory location (to no memory)
- Rust prevents this at compile time
```rust
fn main() {
  let ref_to_nothing = dangle();
}

fn dangle() -> &String { // this func returns a ref to a string
  let s = String::from("hello"); // allocates memory in the heap

  &s // returns a ref to the string s
} // s goes out of scope and the value is dropped.
// this is when &s becomes invalid
```
- rust throws an error at compile time
```docker
this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```
- One way to address this is to claim ownership of the string by returning the value
```rust
fn dangle() -> String {
  let s = String::from("hello");
  
  s
}
```

## Slice Type
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection
- It's a kind of reference, hence no ownership

### String Slices
- a ref to a part of a string
- denoted by `&str`
```rust
let s = String::from("Hello World");

let hello = &s[0..5];
let world = &s[6..1];
```
- here hello and world variables are references to a part of a string
![](./imgs/slice-1.svg)







