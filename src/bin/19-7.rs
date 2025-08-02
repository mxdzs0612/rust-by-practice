// DST and unsized type
// These concepts are complicated, so we are not going to explain here, but you can find them in The Book https://practice.course.rs/newtype-sized.html#newtype-and-sized:~:text=DST%20and%20unsized,directly%20use%20it .

// 🌟🌟🌟 Array with dynamic length is a Dynamic Sized Type ( DST ), we can't directly use it
// /* Make it work with const generics */
// fn my_function(n: usize) -> [u32; usize] {
//     [123; n]
// }

// fn main() {
//     let arr = my_function();
//     println!("{:?}",arr);
// }

/* Make it work with const generics */
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}

fn main() {
    let arr = my_function::<5>();
    println!("{:?}", arr);
}