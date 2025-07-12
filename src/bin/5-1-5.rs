// 🌟🌟

// // 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = x.clone();
//     println!("{:?}, {:?}", x, y);
// }

// 不要使用 clone，使用 copy 的方式替代
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}