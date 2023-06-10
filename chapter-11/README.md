# Automated Testing in Rust

- you use the `#[test]` attribute to annotate a function as a test
- when you create a rust library, it by default has a module named `test` and one example in `src/lib.rs`
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```
- few assertions examples are :
  * `asset!`
  * `assert_eq!`
  * `assert_neq!`
- add the `should_panic` attribute to the test function to tell the runner to expect a panic with a particular message
```rust
#[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
```
- If the test returns `Result<O, E>`, then it passes when Ok is returned and fails when Err is returned 
- To run the tests linearly, without parallelism, use `cargo test -- --test-threads=1`
- To show data output of a function to console log, use `cargo test -- --show-output`

## Running tests by name
- Somtimes running an entore suite of tests can take a long time, when you only have to test a particular section of your module. To accomplish this, rust provides
an option to run tests by name.
- For the following example,
```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```
- To run the 3rd test case, do
```bash
cargo test one_hundred
```
- this is to run a single test, you cannot add more names after `one_hundred` to run multiple tests
- To run multiple tests, it has to be under a 'tag', in the above case, we have tests `two_and_two` and `three_and_two`, both have `add_` before it
- That's the tag. To run both of these tests, do
```bash
cargo test add
```
- This will run all tests containing the word `add` in it 
- This is called test case filtering
- You can use the attribute, `#[ignore]` to ignore execution of a test case
- To run only the ignored tests, do 
  ```bash
  cargo test -- --ignored
  ```
- To run all tests, use
```bash
cargo test -- --include-ignored
```
## Structuring tests in a project
### Unit Tests
- Isolation tests to esure core logic of your code works
- In the auto generated code, when you create a lib, the `#[cfg(test)]`, the cfg attribute stands for configuration, telling Rust that the following item should be givena  certain configuration

#### Testing private functions
- In Rust, you can test private functions. Consider the following example,
```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```
## Integration Tests
- These are external to your lib, it is when someone uses your lib and runs tests with them
- They can then only call the public functions which are available in the API
- Working units of code when put together could result in a failed integration
- You can start by creating a directory called `tests`, cargo knows to look for integration tests in this directory
- Consider the below example,
```bash
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
- The file could look something like,
```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
- Each file in the tests directory is a spearate crate
- We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test
```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00sh
```
- The 3 sections of the output include unit tests, integration tests and doc tests
- If a unit test fails, there won't be any output for integration and doc tests, since those tests are dependent upon unit tests passing
- If we need to run only one set of integration tests, we just need to do 
```bash
$ cargo test --test <name of file>
```
- Since every file under `tests` dir, is a separate crate in itself, we won't be able to do logic like having some common utilities in a separate file, Rust's test runner would run it
- To avoid this, for instance if we have a file `common.rs` with out utils in it, we can just wee it to a file like `common/mod.rs`
- Naming the file this way tells Rust not to treat the common module as an integration test file. So we can now do something like:
```rust
- use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
- the module declaration is the same as for a lib

### Integration tests for Binary crates
- Integration tests are only for library crates since they expose the functions and you can't do that with a binary crate
- Using that structure, integration tests can test the library crate with use to make the important functionality available.
- If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
