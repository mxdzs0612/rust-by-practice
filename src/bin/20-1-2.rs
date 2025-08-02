// 🌟

// // Make it work
// fn main() {
//     let b = Box::new("Hello");
//     print_boxed_string(b);
// }

// fn print_boxed_string(b : _) {
//     println!("{}", b);
// }


// Make it work
fn main() {
    let b = Box::new("Hello");
    print_boxed_string(b);
}

fn print_boxed_string(b : Box<&str>) {
    println!("{}", b);
}