// 🌟
// 可变性
// 当所有权转移时，可变性也可以随之改变。

// fn main() {
//     let s = String::from("hello, ");
    
//     // 只修改下面这行代码 !
//     let s1 = s;

//     s1.push_str("world")
// }


fn main() {
    let s = String::from("hello, ");
    
    // 只修改下面这行代码 !
    let mut s1 = s;

    s1.push_str("world")
}