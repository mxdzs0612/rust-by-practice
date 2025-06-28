// 🌟🌟 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方
// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn()
// }

// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {}

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    // 另两种方法是 panic 和 loop，上一题用过了
    todo!()
}
