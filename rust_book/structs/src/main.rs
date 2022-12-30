// we allow the struct to be displayed using Debug
#[derive(Debug)]
struct User {
    active: bool,
    // right now we can't use &str to store the reference of a String
    // because we cannot be sure if the reference is valid as long as the struct is
    // however String has its own ownership, so it will be active until the struct last use
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // this is a method
    // it works like a function but it accepts an struct object
    // &self => self: &Self -> rectangle: &Rectangle
    // we don't want to take ownership of the parameter "&"
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated functions aren't methods since they don't accept self as first parameter
    // this is like String::new() and usually are used to create an instance of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        // we can't pass the String literal since it is a reference? or a slice string
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // ? we shouldn't be able to use user1 since we have move user1 to user2
    println!("{}", user1.email);
    println!("{}", user2.email);

    // we can create a function that accepts a Color and it won't accept a Point even when the types are the same
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // * we can't use user1 since it was moved before
    // with :? we are able to display an struct using Debug
    // :#? places a line for each attribute
    println!("User: {:#?}", user2);

    // we can also use dbg! which is a macro like println but it takes ownership of the parameter
    // so we pass the reference instead, it also returns the param
    dbg!(&user2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{:?}", Rectangle::square(20));
}
