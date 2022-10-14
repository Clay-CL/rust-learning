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
