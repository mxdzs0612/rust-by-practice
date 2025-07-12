// 🌟



// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(s)
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }


// 修复错误
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);
}

fn push_str(s: &mut String) {
    s.push_str("world")
}