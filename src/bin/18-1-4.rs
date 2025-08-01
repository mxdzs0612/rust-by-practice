// Type inferred
// The following four closures has no difference in input and return types.

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;
// 🌟
// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));

//     /* Make it work, only change the following line */
//     let n = example_closure(5);
// }

fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    /* Make it work, only change the following line */
    let n = example_closure("5".to_string());
}