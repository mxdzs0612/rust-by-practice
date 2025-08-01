// &'static and T: 'static
// 'static 是一个 Rust 保留的生命周期名称，在之前我们可能已经见过好几次了:

// // 引用的生命周期是 'static :
// let s: &'static str = "hello world";

// // 'static 也可以用于特征约束中:
// fn generic<T>(x: T) where T: 'static {}
// 虽然它们都是 'static，但是也稍有不同。

// &'static
// 作为一个引用生命周期，&'static 说明该引用指向的数据可以跟程序活得一样久，但是该引用的生命周期依然有可能被强转为一个更短的生命周期。

// 🌟🌟 有好几种方法可以将一个变量标记为 'static 生命周期, 其中两种都是和保存在二进制文件中相关( 例如字符串字面量就是保存在二进制文件中，它的生命周期是 'static )。

// /* 使用两种方法填空 */
// fn main() {
//     __;
//     need_static(v);

//     println!("Success!")
// }

// fn need_static(r : &'static str) {
//     assert_eq!(r, "hello");
// }


/* 使用两种方法填空 */
fn main() {
    let v = "hello";
    // const v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}