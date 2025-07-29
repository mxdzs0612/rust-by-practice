// From/Into
// From 特征允许让一个类型定义如何基于另一个类型来创建自己，因此它提供了一个很方便的类型转换的方式。

// From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：只要实现了 impl From<T> for U， 就可以使用以下两个方法: let u: U = U::from(T) 和 let u:U = T.into()，前者由 From 特征提供，而后者由自动实现的 Into 特征提供。

// 需要注意的是，当使用 into 方法时，你需要进行显式地类型标注，因为编译器很可能无法帮我们推导出所需的类型。

// 来看一个例子，我们可以简单的将 &str 转换成 String

// fn main() {
//     let my_str = "hello";

//     // 以下三个转换都依赖于一个事实：String 实现了 From<&str> 特征
//     let string1 = String::from(my_str);
//     let string2 = my_str.to_string();
//     // 这里需要显式地类型标注
//     let string3: String = my_str.into();
// }
// 这种转换可以发生是因为标准库已经帮我们实现了 From 特征： impl From<&'_ str> for String。你还可以在这里)找到其它实现 From 特征的常用类型。

// 🌟🌟🌟
// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);

//     // 使用两种方式修复错误
//     // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
//     // 2. 上一章节中介绍过的某个关键字
//     let i3: i32 = 'a'.into();

//     // 使用两种方法来解决错误
//     let s: String = 'a' as String;

//     println!("Success!")
// }

fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // 使用两种方式修复错误
    // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
    // 2. 上一章节中介绍过的某个关键字
    let i3: u32 = 'a'.into();

    // 使用两种方法来解决错误
    let s: String = 'a'.into();

    println!("Success!")
}

// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);

//     let i3: i32 = 'a' as i32;

//     let s: String = String::from('a');
// }