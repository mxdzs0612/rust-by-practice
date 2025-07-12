// 引用和借用 所有权
// 🌟🌟

// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = x;
//     println!("{},{}",x,y);
// }
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    // let y = x.to_owned();
    // let y = x.clone();
    let y = &x;
    println!("{}, {}", x, y);
}
