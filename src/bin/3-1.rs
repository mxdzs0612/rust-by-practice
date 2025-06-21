// 绑定和可变性
// 🌟 变量只有在初始化后才能被使用
// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let x: i32; // 未初始化，但被使用
//     let y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x); 
// }

// 修复下面代码的错误并尽可能少的修改
fn main() {
    let x: i32 = 5;
    let y: i32;
    println!("x is equal to {}", x); 
}