// 🌟🌟 Ok: 从可变对象借用不可变
// // 下面的代码没有任何错误
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s);
    
//     s.push_str("world");
// }

// fn borrow_object(s: &String) {}


// 下面的代码没有任何错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");
}

fn borrow_object(s: &String) {}