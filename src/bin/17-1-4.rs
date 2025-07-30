// 🌟🌟🌟
// /* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>() -> &'a String { 
//     &String::from("foo") 
// }

// fn main() {
// }

/* 使用三种方法修复下面的错误  */
fn invalid_output() -> String { 
    String::from("foo") 
}

fn main() {
}
// fn invalid_output() -> &'static str { 
//     "foo"
// }

// fn main() {}
// fn invalid_output<'a>(s: &'a String) -> &'a String { 
//     s
// }

// fn main() {}