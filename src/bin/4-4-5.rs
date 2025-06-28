// 🌟🌟
// fn main() {
//     // 填空
//     let b = __;

//     let _v = match b {
//         true => 1,
//         // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic")
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }

fn main() {
    // 填空
    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
