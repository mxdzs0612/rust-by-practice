// 🌟🌟 Trait is also an unsized type
// /* Make it work in two ways */
// use std::fmt::Display;
// fn foobar(thing: Display) {}    

// fn main() {
// }

/* Make it work in two ways */
use std::fmt::Display;
fn foobar(thing: &dyn Display) {}

fn foobar2(thing: Box<dyn Display>) {}    

fn main() {
}