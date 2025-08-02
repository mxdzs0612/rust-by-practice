// 🌟

// // Make it work
// fn main() {
//     // Create a box `b` with an array [1, 2, 3, 4, 5]
//     // Print each integer in `b`
// }


// Make it work
fn main() {
    // Create a box `b` with an array [1, 2, 3, 4, 5]
    // Print each integer in `b`
    let arr = Box::new([1, 2, 3, 4, 5]);
    for i in  arr.iter() {
        print!("{}", i);
    }
}