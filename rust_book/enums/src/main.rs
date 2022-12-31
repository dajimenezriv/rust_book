enum IpAddr {
    // we can store values or tuples or whatever inside our enums
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can have methods
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// in rust we don't have the value null
// enum Option<T> {
//     None,
//     Some(T),
// }

// this function, if option is some, we add one, otherwise we return none
// we need to match all possible values
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ => (), // * it's like default, we do nothing ()
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // we need to specify what to do when Option is Some or None
    // let sum = x + y; // ! error

    // but if we don't want to match all options we can do // * if let
    let config_max = Some(3u8);
    // if config_max matches Some(max) we perform the println!
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
