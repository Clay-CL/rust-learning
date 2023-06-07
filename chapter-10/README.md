# Generics, Traits and Liftimes

- the following code can be optimized using generics
```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```
- the common purpose of the two functions is to get the largest item in a slice
```rust
fn largest<T>(lst: &[T]) -> &T {
  let mut largest = &list[0];
  
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  
  largest;
}
```
- the above code would throw an error, due to the fact that it won't work for all possible values of **T**
- To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
- The code will compile when you change the signature to `fn largest<T: std::cmp::PartialOrd`

## In Structs
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
- This code also won't compile since we have a single type for both members, you'd get error `E0308`
- To get multiple generic types, you can do :
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```
## with Enums
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## with Methods
```rust
struct Point<T> {
	x: T,
	y: T
}

impl<T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
```

- since we can have as many `impl` blocks, we can have another with just pointing to `f32`
```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```
- using generic types won't make your program run any slower than it would with concrete types
- Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled

## Traits
- traits are like interfaces from java
- shared behaviour can be defined with these in an abstract way
```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```
- Now to implement this trait, we do the following
```rust
pub struct NewsArticle {
  pub headline: String,
 	pub location: String,
	pub author: String,
	pub content: String 
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

// different struct
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
- let's say these were in a lib crate called `aggregator`
- to sue them in some other crate, you need to bring the trait and the type in the scope
```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
- One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate
- we can implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality, because the type Tweet is local to our aggregator crate. 
- We can also implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator crate
- But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate
- This restriction is part of something called **coherence**
- This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use

### Default implementation
- just like open functions in kotlin, we can have default implementations to trait methods
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
- when wanting to use a default implementation for a type, one can simply do 
```rust
impl Summary for NewsArticle {}
```
- methods within the trait can also call methods which don't have any implementation
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
- Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

### Traits as parameters
- just likek you can pass interfaces as arguments in kotlin, traits can also be passed
```rust
pb fn notify(item: &impl Summary) {
  println!("Breaking news ! {}", item.summarize());
}
```
- item can be any struct type which implements the `Summary` trait, hence it's demarqued as `&impl Summary`
- when called with any other type, it won't compile

### Trait bound syntax
- The above snippet can also be written as :
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

#### Multiple trait bounds
- In kotlin, a class can implement multiple interfaces. Similary in rust, a type can implement multiple traits
- Say in the previous code snippet, we want item to be able to do both display and summary. To get this done, the function can mention the type needs to include both `Display` and `Summary` traits, can be accomplished using the `+` operator
```rust
pub fn notify(item: &(impl Summary + Display)) { 
  // something ... 
}
```
# Generics, Traits and Liftimes

- the following code can be optimized using generics
```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
### Returning types that implement traits
- returning `impl <Trait>` for a function helps in returning an implementation of the trait without naming a concrete type
- and no, we won't be able to return different types voia a switch case, this only works for single return types due to restrictions on how the compiler is implemented

### Conditionally implement methods
- consider a type `Pair<T>`
```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
```
- Now let's say the type T has implementations of PartialOrd and Display, for those types, you can have a special implementation
```rust
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

## Lifetimes
- another kind of generic that ensures a type has the behaviour we want , i.e ensuring that refs are valid as long as we need them to
- most of the time lifetimes for refs are implicit just like for types
- types need to be annotated when multiple types are possible, similarly lifetimes need to be annotated

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- possible syntax
```rust
&'a x; // lifetime associated with reference x
&'a mut x; // lifetime associated with a mutable reference x
```
### Rules
- These are called lifetime elision
- The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
- For multiple input lifetime params, if one of them is `self`, then the lifetime of `self` is assigned to all output lifetime params

### Static lifetime
- Denoted by `&'static`
- It denites that the specificed reference can live for the entire duration of the program
- All string literals have static lifetimes

