# Packages, Crates and Modules


## `use` keyword
- this can bring a module into scope
```rust
mod m1 {
  pub mod host {
    pub fn add() {}
  }
}

use crate::m1::host;

pub fn eat() {
  host::add();
}
```

- this is like creating a symlink in a filesystem
- `use` keyword is bound by scope

Consider the following code snippet, it would not compile since eat() is not in the same scope as where the use was used

```rust
mod m1 {
  pub mod host {
    pub fn add() {}
  }
}

use crate::m1::host;

mod customer {
  // this would work
  // use crate::m1::host;
  
  pub fn eat() {
    host::add();
  }
}
```

- it's better to use full path when using structs, enums, etc as shown below
```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```
- there's one exception to this, it's when there are two items having the same name into scope with `use` statements
```rust
use std::fmt;
use std::io;

fn func1() -> fmt::Result {
   // someting
}

fn func2() -> io::Result<()> {
  // something else
}
```
- another way to do this would be to use the `as` keyword
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn func1() -> Result {
   // someting
}

fn func2() -> IoResult<()> {
  // something else
}
```
- combining `pub` and `use` enables to refer to an item publicly, else it's private to the scope it's used in
- This is called **re-exporting**, since we're not only bringing this item to scope, we're also allowing others to use it when imported
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
- Before external code would need to do `restaurant::front_of_house::hosting::add_to_waitlist()`, now we can just do `restaurant::hosting::add_to_waitlist()`

## Using External packages
- packages and their dependencies are downloaded from `crates.io`
- Dependencies of projects, i.e packages you want to use, needs to be defined in `Cargo.toml`

```toml
rand = "0.8.5"
```
in code you would use it like
```rust
use rand::Rng;

fn main() {
  let secret_num = rand::thread_rng().gen_range(1..=100);
}
```
- `std` library is also an external package but is shipped along with the Rust language
- when trying to use many items, you can clump them up
```rust
// when trying to do 
use std::cmp::Ordering;
use std::io;

// you can do 
use std::{cmp::Ordering, io};
```
- this is called nested path
- this can extend to any part of the path
```rust
// when trying to do
use td::io;
use std::io::Write;

// you can also do
use std::io::{self, Write};
```
- you can also import everthing from a module by using * (the `glob` operator)


## Separating Modules into Different Files
- 
