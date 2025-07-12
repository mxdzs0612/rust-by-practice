// 🌟🌟

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.into_bytes();
//     s
// }

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // 将 String 转换成 Vec 类型
    let _s = s.as_bytes();
    s
}