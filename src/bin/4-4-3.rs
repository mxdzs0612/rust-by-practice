// 🌟🌟🌟
// // 用两种方法求解
// fn main() {
//     never_return();
// }

// fn never_return() -> ! {
//     // 实现这个函数，不要修改函数签名!

// }

// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    // 法1：panic!("");
    loop {
        println!("无限循环");
    }
}
