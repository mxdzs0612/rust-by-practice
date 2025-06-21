// 变量作用域
// 🌟🌟
// 修复错误
// fn main() {
//     println!("{}, world", x); 
// }

// fn define_x() {
//     let x = "hello";
// }

// 修复错误
fn main() {
    define_x()
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x); 
}