// 🌟🌟

// // 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");

//     print_str(s);

//     println!("{}", s);
// }

// fn print_str(s: String)  {
//     println!("{}",s)
// }
// 修复错误，不要删除任何代码行
fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}", s)
}