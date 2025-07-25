// 语句与表达式
// 示例
// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// 🌟🌟
// // 使用两种方法让代码工作起来
// fn main() {
//    let v = {
//        let mut x = 1;
//        x += 2
//    };

//    assert_eq!(v, 3);
// }

// 使用两种方法让代码工作起来
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    // 也可以 assert_eq!(v, {});
    assert_eq!(v, 3);
}
