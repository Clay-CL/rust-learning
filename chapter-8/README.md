# Collections in Rust
std lib comes with collections like, vector, string, map

## Vectors
```ust
let v: Vec<i32> = Vec::new();
```

- rust also provides a macro to create a vector
```rust
let v = vec![1, 2, 3];
```

### Updating a vector
```rust
let mut v = vec![1. 2, 3, 4];

v.push(5);
v.push(6);
v.push(7);
```

### Indexing vector elements
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("3rd element is {third}");

let third: Option<&i32> = v.get(2);
match third {
  Some(third) => println!("The 3rd element is {third}");
  None => println!("No element");
}
```

the following code gives rise to an error
```rust
let v = vedc![1, 2, 3, 4, 5];

let d = &v[100]; // causes trouble
let d = v.get(100);
```
- `&v[100]` causes the program to pani, similar to an `ArrayIndexOutofBounds` Exception in java
```bash
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100' 
```
- You also can't have mutable and immutable references in the same scope
```rust
let mut v = vec![1, 2, 3, 4, 5];

let f = &v[0];

v.push(6);

println!("The 1st element is : {f}");
```

### Iterate over elements
```rust
let v = vec![1, 2, 3];

for i in &v {
  println!("{i}");
}
```

- to iterate over mutable refs, we can do 
  ```rust
  let mut v = vec![1, 2, 3];
  
  for i in &mut v {
    *i += 50;
  }
  ```
 - Dropping a vector ensures associated memory of it's elements are also dropped

## Strings
We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

- both str and String are UTF-8 encoded
- When doing an operation like `let s3 = s1 + &s2`, s1 is no longer valid after the concatenation
- When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]

## Lengths
```rust
let hello = String::from("Hola");
```
the above string represents 4 bytes, whereas 

```rust
let hello = String::from("Здравствуйте");
```
represents 24 bytes
- we can't even slice them like
```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```
rust would panic in this case
```bash
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
       Finished dev [unoptimized + debuginfo] target(s) in 0.43s
            Running `target/debug/collections`
            thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
            
```

- to iterate over them, you can use the `.chars()` function
```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

## Hashmaps
```rust
use std::collections::HashMap;

let mut scores - HashMap::new();
scores.insert(String::from("Blue"), 10);

// accessing values
let team_name = String::from("Blue");
let score = scores.get(&tram_name).copied().unwrap_or(0);
```
- The above code handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.
- To iterate over key, value pairs do
```rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```
### Ownership
- types that implement **Copy** trait, ex. `i32`, values are copied into the hashmap
- Otherwise, the values are moved and Ownership transfered to the hashmap
```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
println!("Field name : {field_name}")
```
Output :
```bash
   Compiling hashmaps v0.1.0 (/Users/clay/rust-learning/chapter-8/hashmaps)
error[E0382]: borrow of moved value: `field_name`
  --> src/main.rs:10:29
   |
4  |     let field_name = String::from("Favorite color");
   |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
...
8  |     map.insert(field_name, field_value);
   |                ---------- value moved here
9  |
10 |     println!("Field name : {field_name}")
   |                             ^^^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
```
### Updating a hash map
- Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa: for example, both the Blue team and the Yellow team could have value 10 stored in the scores hash map).
- You can use the `entry` function to check if a key exists or not
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores)
```











