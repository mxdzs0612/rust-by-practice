// into_iter, iter and iter_mut
// In the previous section, we have mentioned that for will apply the into_iter to the collection, and change it into a iterator. However, this is not the only way to convert collections into iterators.

// into_iter, iter, iter_mut, all of them can convert a collection into iterator, but in different ways.

// into_iter consumes the collection, once the collection has been consumed, it is no longer available for reuse, because its ownership has been moved within the loop.
// iter, this borrows each element of the collection through each iteration, thus leaving the collection untouched and available for reuse after the loop
// iter_mut, this mutably borrows each element of the collection, allowing for the collection to be modified in place.
// 🌟
// /* Make it work */
// fn main() {
//     let arr = vec![0; 10];
//     for i in arr {
//         println!("{}", i);
//     }

//     println!("{:?}",arr);
// }

/* Make it work */
fn main() {
    let arr = vec![0; 10];
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("{:?}", arr);
}
