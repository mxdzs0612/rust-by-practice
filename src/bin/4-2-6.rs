// 🌟🌟 单元类型占用的内存大小是多少？
// // 让代码工作：修改 `assert!` 中的 `4`
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 4);

//     println!("Success!")
// }

// 让代码工作：修改 `assert!` 中的 `4`
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
