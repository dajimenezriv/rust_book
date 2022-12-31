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
