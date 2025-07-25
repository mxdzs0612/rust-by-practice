// 🌟🌟
// // 修改以下结构体让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }


// 修改以下结构体让代码工作
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // 不要修改这行代码！
    let p = Point{x: 5, y : "hello".to_string()};
}