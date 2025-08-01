// Iterator adaptors
// Methods allowing you to change one iterator into another iterator are known as iterator adaptors. You can chain multiple iterator adaptors to perform complex actions in a readable way.

// But because all iterators are lazy, you have to call one of the consuming adapters to get results from calls to iterator adapters.

// 🌟🌟
// /* Fill in the blanks */
// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().__.__;

//     assert_eq!(v2, vec![2, 3, 4]);
// }

/* Fill in the blanks */
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
