use std::collections::HashMap;

fn main() {
    /* -------------------------------------------------------------------------- */
    /*                                   Vectors                                  */
    /* -------------------------------------------------------------------------- */
    
    // if we do not provide initial values, we need to anotate the vector type
    let _v: Vec<i32> = Vec::new();
    let mut _v = vec![1, 2, 3];
    _v.push(5);
    // in this case, &_v[1], if the position doesn't exist, the program will panic
    println!("Access second element via indexing: {}", &_v[1]);
    let second: Option<&i32> = _v.get(1);
    match second {
        Some(second) => println!("Access second element via get: {second}"),
        None => println!("There is no second element"),
    }

    // * ownership
    let mut v = vec![1, 2, 3, 4, 5]; // we create a mutable vec
    let first = &v[0]; // now we create a reference to the first position
    // v.push(6); // ! we can't mutate the v because we created a reference of v that is used after
    println!("The first element is: {first}");
    v.push(6); // however, now, since first is no longer used we can mutate v

    // * dereference
    let mut v = vec![100, 32, 57];
    // if we want to change the values, first we need to get the mutable reference
    for i in &mut v {
        // now we need to dereference to get the value of &v
        *i += 50;
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Strings                                  */
    /* -------------------------------------------------------------------------- */

    let mut hello = String::from("السلام عليكم");
    hello.push_str("foo");
    println!("{hello}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // ? why we can still use s2? and the ownership?
    println!("s2 is {s2}");

    // combine strings
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    // s1 becomes invalid
    // * fn add(self, s: &str) -> String {
    // * str is a string slice and String it's like a data type
    // but the reference of s2 is &String, not &str
    // the compiler can coerce the &String argument into a &str
    // add doesn't take ownership of s2, but it takes ownership of s1, which becomes unavailable
    let s3 = s1 + ", " + &s2 + "!";
    println!("{s3}");

    // Rust strings don’t support indexing because it contains bytes not characters
    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte
    // A String is a wrapper over a Vec<u8>
    // * https://doc.rust-lang.org/book/ch08-02-strings.html

    /* -------------------------------------------------------------------------- */
    /*                                  Hash Maps                                 */
    /* -------------------------------------------------------------------------- */

    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    // if we add team_name to insert, then it becomes invalid
    // if we insert references we have to be careful that those references are valid as long as the hash map is
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites
    // the get method returns an Option<&V>
    // calling copied we get an Option<V> instead of Option<&V>
    // unwrap_or(0) returns the value if it exists or 0
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // if the entry doesn't exist, insert
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // update entry based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        // this a reference
        let count = map.entry(word).or_insert(0);
        // to access the value we must dereference
        *count += 1;
    }

    println!("{:?}", map);
}
