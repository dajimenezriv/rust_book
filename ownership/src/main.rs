// the parameter needs to be a reference, otherwise we will deallocate the local variable
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // empty space
        if item == b' ' {
            return &s[..i];
        }
    }
    // in case there is only one word, we return the length
    &s[..]
}

fn main() {
    let /* mut */ s = String::from("hello world");
    let word = first_word(&s);
    // we can't clear the string because there is an inmutable reference to that string in word
    // so if we try to do a mutable borrow it will throw an error
    // s.clear(); // ! error
    println!("the first word is: {}", word);
}
