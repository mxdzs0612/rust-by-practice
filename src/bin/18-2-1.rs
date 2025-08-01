// Iterator
// The iterator pattern allows us to perform some tasks on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.

// for and iterator
// fn main() {
//     let v = vec![1, 2, 3];
//     for x in v {
//         println!("{}",x)
//     }
// }
// In the code above, You may consider for as a simple loop, but actually it is iterating over a iterator.

// By default for will apply the into_iter to the collection, and change it into a iterator. As a result, the following code is equivalent to previous one:

// fn main() {
//     let v = vec![1, 2, 3];
//     for x in v.into_iter() {
//         println!("{}",x)
//     }
// }
// 🌟
// /* Refactoring the following code using iterators */
// fn main() {
//     let arr = [0; 10];
//     for i in 0..arr.len() {
//         println!("{}",arr[i]);
//     }
// }

/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for i in arr {
        println!("{}", i);
    }
}