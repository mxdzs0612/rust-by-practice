// next method
// All iterators implement a trait named Iterator that is defined in the standard library:

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // Methods with default implementations elided
// }
// And we can call the next method on iterators directly.

// 🌟🌟
// /* Fill the blanks and fix the errors.
// Using two ways if possible */
// fn main() {
//     let v1 = vec![1, 2];

//     assert_eq!(v1.next(), __);
//     assert_eq!(v1.next(), __);
//     assert_eq!(v1.next(), __);
// }


/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let v1 = vec![1, 2];
    let mut v1 = v1.into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}

// fn main() {
//     let v1 = vec![1, 2];

//     // borrowing
//     let mut v1_iter = v1.iter();

//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), None);
// }