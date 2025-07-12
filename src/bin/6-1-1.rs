// 字符串
// 字符串字面量的类型是 &str， 例如 let s: &str = "hello, world" 中的 "hello, world" 的类型就是 &str。

// str 和 &str
// 🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代

// // 修复错误，不要新增代码行
// fn main() {
//     let s: str = "hello, world";
// }

// 修复错误，不要新增代码行
fn main() {
    let s: &str = "hello, world";
}