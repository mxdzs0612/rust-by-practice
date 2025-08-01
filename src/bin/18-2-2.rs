// 🌟 One of the easiest ways to create an iterator is to use the range notion: a..b.
// /* Fill in the blank */
// fn main() {
//     let mut v = Vec::new();
//     for n in __ {
//        v.push(n);
//     }

//     assert_eq!(v.len(), 100);
// }

/* Fill in the blank */
fn main() {
    let mut v = Vec::new();
    for n in 0..100 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);
}