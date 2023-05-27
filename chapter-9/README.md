# Error Handling in Rust
- Just like in kotlin, to cause an exception one would use the keyword `throw`
- In case of rust, it would be the `panic!` macro
```rust
fn main() {
  panic!("Testing panic");
}
```
- set RUST_BACKTRACE=1 to get backtraces to backtrack how the panic occurred
- To get this level of backtrace, this is only possible with debug builds, when using the `--release` flag with `cargo build` or `cargo run` it would be disabled

## Recoverable errors with `Result`
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- just like types of exceptions, we have types of `Err`
Ex:
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let file_name = "hello.txt"
  let res = File::open(file_name);
  let file = match res {
    Ok(file_) => file_,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(file_name) {
        Ok(fc) => fc,
        Error(e) => panic!("Error in creating {file_name} : {:?}", e),
      },
      other_error => {
        panic!("Error in opening file : {:?}", other_error);
      }
    }
  };
}
```
- the abpve example has a lot of march statements, alternative to this is closures [in Chapter 13](../chapter-13/README.md)
- the function `unwrap()` can attch to a function and there is no need to handle the match ladder, it would panic upon error
```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```
- Similarly, there's the `expect` function that let's you choose the panic message to print
```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```
### Error propagation
- When an error occurs in a function, one can return the error without handling it within the function itself
Ex:
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```
- There's an easier way to do this in rust using the `?` operator. The code would then look like:
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
- If the value of the expression is of type `Ok`, then the program continues, else the `Err` is returned
- this cannot be used in a function whose return type is not `Result<T, E>` or `Option`
```rust
use std::fs::File;

fn main() {
  let greeting_file = File::open("hello.txt")?;
}
```
You'll end up getting a compile time error:
```bash
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` due to previous error
```
- This error points out that we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.
- If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test. Because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.
- In cases of bad states, such as when making API calls, if the execution is not able to continue and there is nothing the program can do to fix it, then you need to let the program panic.
- However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.
- When your code performs an operation that could put a user at risk if it’s called using invalid values, your code should verify the values are valid first and panic if the values aren’t valid. This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem
- One could define a type which would panic in case of an invalid state
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
- 




















