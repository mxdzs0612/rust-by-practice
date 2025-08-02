// 🌟

// // Make it work
// fn main() {
//     let b1 = Box::new(5);
//     let b2 = b1;
//     assert_eq!(_, 5);

//     println!("Success!");
// }


// Make it work
fn main() {
    let b1 = Box::new(5);
    let b2 = b1;
    assert_eq!(*b2, 5);

    println!("Success!");
}