// 🌟🌟 We can also manually implement Debug trait for our types

// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);


// fn main() {    
//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?

//     /* Make it print: Now 7 will print! */
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }

use std::fmt::Debug;


// #[derive(Debug)]
struct Structure(i32);

// #[derive(Debug)]
struct Deep(Structure);
impl Debug for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.0)
    }
}


fn main() {    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}