// 深入生命周期
// 特征约束
// 就像泛型类型可以有约束一样，生命周期也可以有约束 ，如下所示：

// T: 'a，所有引用在 T 必须超过生命周期 'a
// T: Trait + 'a: T 必须实现特征 Trait 并且所有引用在 T 必须超过生命周期 'a
// 示例
// use std::fmt::Debug; // 特征约束使用

// #[derive(Debug)]
// struct Ref<'a, T: 'a>(&'a T);
// // `Ref` 包含对泛型类型 `T` 的引用，该泛型类型具有
// // 未知的生命周期 `'a`. `T` 是约定任何
// // 引用在 `T` 必须大于 `'a` 。此外，在生命周期
// // 里 `Ref` 不能超过 `'a`。

// // 使用 `Debug` 特征打印的通用函数。
// fn print<T>(t: T) where
//     T: Debug {
//     println!("`print`: t is {:?}", t);
// }

// // 这里引用 `T` 使用 where `T` 实现
// // `Debug` 和所有引用 `T` 都要比 `'a` 长
// // 此外，`'a`必须要比函数声明周期长
// fn print_ref<'a, T>(t: &'a T) where
//     T: Debug + 'a {
//     println!("`print_ref`: t is {:?}", t);
// }

// fn main() {
//     let x = 7;
//     let ref_x = Ref(&x);

//     print_ref(&ref_x);
//     print(ref_x);
// }
// 🌟
// /* 使用生命周期注释结构体
// 1. `r` 和 `s` 必须是不同生命周期
// 2. `s` 的生命周期需要大于 'r'
// */
// struct DoubleRef<T> {
//     r: &T,
//     s: &T
// }
// fn main() {
//     println!("Success!")
// }

/* 使用生命周期注释结构体
1. `r` 和 `s` 必须是不同生命周期
2. `s` 的生命周期需要大于 'r'
*/
struct DoubleRef<'a, T: 'a> {
    r: &'a T,
    s: &'a T
}
fn main() {
    println!("Success!")
}

// struct DoubleRef<'a,'b:'a, T> {
//     r: &'a T,
//     s: &'b T
// }
// fn main() {
//     println!("Success!")
// }