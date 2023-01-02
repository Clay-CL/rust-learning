# Structs in Rust
- just like in C and C++ rust has structs
```rust
struct User {
  active: bool,
  username: String,
  email: String,
  count: u64
}

fn main() {
  let user = User {
    email: String::from("test@email.com"),
    username: String::from("some_username"),
    active: true,
    count: 1,
  };
}
```

- members of the struct can be accessed like in kotlin, java
```rust
fn main() {
   let mut user = User {
    email: String::from("test@email.com"),
    username: String::from("some_username"),
    active: true,
    count: 1,
  };
  user.email = String::from("test2@email.com");
}
```

- a function can return a struct
```rust
fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    count: 1,
  };
}
```
- one can also do field shorthand
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
- there is also something called tuple structs similar to tuples
```rust
struct Color(i32, i32, i32);
struct ColorAlpha(i32, i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let black50 = Color(0, 0, 0, 127);
}
```
- unit like structs can also exist
```rust
struct Something;

fn main() {
  let something = Something;
}
```

## Ownership of Struct data
- `&str` could have been used, but `String` is used so that the struct can own all the data


## Methods
- functions defined in the context of a struct
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

- Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.
- following is the same
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```


