// 🌟🌟

// // 填空
// fn print_array<__>(__) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }

use std::fmt::Debug;

// 填空
fn print_array<T: Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
