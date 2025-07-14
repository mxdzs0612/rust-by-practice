// 🌟🌟 我们可以使用 String::from 或 to_string 将 &str 转换成 String 类型
// // 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = s;
// }


// 使用两种方法来解决错误，不要新增代码行
fn main() {
    let s = "hello, world".to_string();
    // let s1: &str = &s;
    let s1: String = s;
}

// fn main() {
//     let s = "hello, world";
//     let s1: &str = s;
// }