// Methods that Consume the Iterator
// The Iterator trait has a number of methods with default implementations provided by the standard library.

// Consuming adaptors
// Some of these methods call the method nextto use up the iterator, so they are called consuming adaptors.

// 🌟🌟

// /* Fill in the blank and fix the errors */
// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
//     let total = v1_iter.sum();

//     assert_eq!(total, __);

//     println!("{:?}, {:?}",v1, v1_iter);
// }



/* Fill in the blank and fix the errors */
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}, {:?}",v1, v1);
}