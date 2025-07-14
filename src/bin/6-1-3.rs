// String
// String 是定义在标准库中的类型，分配在堆上，可以动态的增长。它的底层存储是动态字节数组的方式( Vec<u8> )，但是与字节数组不同，String 是 UTF-8 编码。

// 🌟
// // 填空
// fn main() {
//     let mut s = __;
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");
// }


// 填空
fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}