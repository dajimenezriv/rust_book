# Fourth Chapter: Ownership

## What is Ownership?

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

## References and Borrowing

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

We are referencing. The opossite is dereferencing, using the operator *.<br>

![Reference](https://raw.githubusercontent.com/dajimenezriv/rust_book/main/ownership/ownership.svg)

We can update a mutable reference.

```rust
fn main() {
  let mut s = String::from("hello");
  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

### Slices

Explained in ownership folder.

```rust
// a string slice is a reference to part of a String
let s = String::from("hello world");
// hello is a reference to a portion of the String
let hello = &s[0..5];
let world = &s[6..11];
```
