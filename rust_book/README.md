# Rust Book

## First Chapter: Getting Started

```rust
// "!" means macro
println!("Hello World!");
```

```bash
cargo build
# our executables are saved at target/debug
# Cargo.lock keeps track of the exact versions of dependencies in your project
cargo run # also builds
cargo check # make sure the program compiles but it doesn't produce an executable
cargo build --release # build the program with optimizations, but compilation is slower
```

## Second Chapter: Basics

```rust
// by default, Rust has a set of items defined in the standard library that it brings into the scope of every program
// this set is called the prelude
use std::io; // the i/o library comes from the standard library

// variables are immutable by default
let apples = 5;
let mut apples = 5;

// a new instance of a String
// the :: syntax in the ::new line indicates that new is an associated function of the String type
String::new();

// read_line returns a Result (enum) and it has two possibilities
// Ok: contains the read value
// Err: contains information about how or why the operation failed
// an instance of Result has the expect method
// expect will cause the program to crash and display the message
io::stdin()
  .read_line(&mut guess)
  .expect("Failed to read line");

// variables inside a string
println!("You guessed: {guess}");

// range
(1..=100) // both inclusive
```

### Cargo.lock

Our Cargo.toml contains the version of our dependencies.<br>
However, rand = "0.8.5" means rand = "^0.8.5", so, how do we know the version used?<br>
Cargo.lock contains the version of the compiled program. Once it is first compiled, everytime we compile it again we reuse the same versions.<br>
For example, if [crates.io](https://crates.io/) has new version, we can update our version using:

```bash
cargo update
```

```toml
[dependencies]
rand = "0.8.5"
```

## Third Chapter: Common Concepts

### Constants

Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Shadowing

```javascript
// we shadow when we want to change the type variable for example
// also, as now, the first variable is mutable, while the second is not
let mut guess = String::new();
// u32 is an unsigned integer (positive)
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

### Data Types

```rust
// it means that the number is unsigned of 8 bits
let number = 58u8;

// tuples
// we have multiple forms to access the individual elements
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;

// arrays are useful when you want your data allocated on the stack rather than the heap
// always fixed length
let a = [1, 2, 3, 4, 5];
```

### Functions

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

- <b>Statements</b> are instructions that perform some action and do not return a value.
- <b>Expressions</b> evaluate to a resultant value. Let’s look at some examples.

```rust
// statement, doesn't return anything
let x = (let y = 6); // error

let y = {
  // statement
  let x = 3;
  // expression
  // expressions do not include ending semicolons
  // if you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
  x + 1
};
```

### Control Flow

```rust
// using if in a let statement
let number = if condition { 5 } else { 6 };

// returning values in a loop
let result = loop {
  counter += 1;

  if counter == 10 {
    break counter * 2;
  }
};

// labeling loops
'counting_up: loop {
  println!("count = {count}");
  let mut remaining = 10;

  loop {
    println!("remaining = {remaining}");
    if remaining == 9 {
      break;
    }
    if count == 2 {
      // otherwise it will break the innermost loop
      break 'counting_up;
    }
    remaining -= 1;
  }

  count += 1;
}

// while
while number != 0 {
  println!("{number}!");
  number -= 1;
}

// iterating a collection
let a = [10, 20, 30, 40, 50];
for element in a {
  println!("the value is: {element}");
}

// range
for number in (1..4).rev() {
  println!("{number}!");
}
```

## Fourth Chapter: Ownership

### What is Ownership?

```rust
// "hello" is a String literal, while s becomes an String
// Strings can be mutated while String literals not
let s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```

This is because Strings are allocated in the heap, and they use an unknown size memory.<br>
They become slow while String literals are fasters because they are inmutable.<br>
We need a way to return this allocated memory to the heap when we are done with our String.<br>
In C this is done with allocate and free.<br>
Rust calls drop (free) automatically at the closing curly bracket.<br>
In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII).

```rust
{
  let s = String::from("hello");
} // drop()
```

An string is actually a pointer, a length (bytes used) and a capacity (bytes received from the allocator).<br>
Rust will never automatically create "deep" copies of your data.<br>
This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.<br>
To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.

```rust
let s1 = String::from("hello");
let s2 = s1;
let s2 = s1.clone(); // deep copy of the heap

println!("{}, world!", s1); // error
```

If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.<br>
Here are some of the types that implement Copy:
All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

For example, if we pass a String to a function, then the local variable passed to the functions is dropped.

### References and Borrowing

Instead of passing the string pointer itself, we can pass a reference:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // we can still use s1
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
