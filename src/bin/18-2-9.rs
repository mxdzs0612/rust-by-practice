// Collect
// Other than converting a collection into an iterator, we can also collect the result values into a collection, collect will consume the iterator.

// 🌟🌟
// /* Make it work */
// use std::collections::HashMap;
// fn main() {
//     let names = [("sunface",18), ("sunfei",18)];
//     let folks: HashMap<_, _> = names.into_iter().collect();

//     println!("{:?}",folks);

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2 = v1.iter().collect();

//     assert_eq!(v2, vec![1, 2, 3]);
// }

/* Make it work */
use std::collections::HashMap;
fn main() {
    let names = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.into_iter().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}
