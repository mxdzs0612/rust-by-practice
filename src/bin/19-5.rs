// Type alias
// Type alias is important to improve the readability of our code.

// type Thunk = Box<dyn Fn() + Send + 'static>;

// let f: Thunk = Box::new(|| println!("hi"));

// fn takes_long_type(f: Thunk) {
//     // --snip--
// }

// fn returns_long_type() -> Thunk {
//     // --snip--
// }
// type Result<T> = std::result::Result<T, std::io::Error>;
// And Unlike newtype, type alias don't create new types, so the following code is valid:

// type Meters = u32;

// let x: u32 = 5;
// let y: Meters = 5;

// println!("x + y = {}", x + y);
// 🌟
// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
// }

// /* Fill in the blank */
// __

// fn main() {
//     // We can refer to each variant via its alias, not its long and inconvenient
//     // name.
//     let x = Operations::Add;
// }

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

/* Fill in the blank */
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}