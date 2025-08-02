// 🌟
// // Make it work
// fn main() {
//     // Create a new box `b` that contains the integer 5
//     assert_eq!(*b, 5);

//     println!("Success!");
// }

// Make it work
fn main() {
    // Create a new box `b` that contains the integer 5
    let b = Box::new(5);
    assert_eq!(*b, 5);

    println!("Success!");
}