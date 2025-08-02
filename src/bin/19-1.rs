// newtype and Sized
// Newtype
// The orphan rule tells us that we are allowed to implement a trait on a type as long as either the trait or the type are local to our crate.

// The newtype pattern can help us get around this restriction, which involves creating a new type in a tuple struct.

// 🌟
// use std::fmt;

// /* Define the Wrapper type */
// __;

// // Display is an external trait
// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     // Vec is an external type, so you cannot implement Display trait on Vec type
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

use std::fmt;

/* Define the Wrapper type */
struct Wrapper(Vec<String>);

// Display is an external trait
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    // Vec is an external type, so you cannot implement Display trait on Vec type
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}