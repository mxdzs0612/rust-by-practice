// 🌟🌟 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用

// // 修复错误，尽量少地修改代码
// // 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;

//     match r {
//        &mut value => value.push_str(" world!") 
//     }
// }

// 修复错误，尽量少地修改代码
// 不要移除任何代码行
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") 
    }
}