// Debug and Display
// All types which want to be printable must implement the std::fmt formatting trait: std::fmt::Debug or std::fmt::Display.

// Automatic implementations are only provided for types such as in the std library. All others have to be manually implemented.

// Debug
// The implementation of Debug is very straightforward: All types can derive the std::fmt::Debug implementation. This is not true for std::fmt::Display which must be manually implemented.

// {:?} must be used to print out the type which has implemented the Debug trait.


// // This structure cannot be printed either with `fmt::Display` or
// // with `fmt::Debug`.
// struct UnPrintable(i32);

// // To make this struct printable with `fmt::Debug`, we can derive the automatic implementations provided by Rust
// #[derive(Debug)]
// struct DebugPrintable(i32);
// 🌟

// /* Fill in the blanks and Fix the errors */
// struct Structure(i32);

// fn main() {
//     // Types in std and Rust have implemented the fmt::Debug trait
//     println!("__ months in a year.", 12);

//     println!("Now __ will print!", Structure(3));
// }


/* Fill in the blanks and Fix the errors */
#[derive(Debug)]
struct Structure(i32);

fn main() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}