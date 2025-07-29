// 孤儿原则
// 关于孤儿原则的详细介绍请参见特征定义与实现的位置孤儿规则 https://course.rs/basic/trait/trait#%E7%89%B9%E5%BE%81%E5%AE%9A%E4%B9%89%E4%B8%8E%E5%AE%9E%E7%8E%B0%E7%9A%84%E4%BD%8D%E7%BD%AE%E5%AD%A4%E5%84%BF%E8%A7%84%E5%88%99 和 在外部类型上实现外部特征 https://course.rs/basic/trait/advance-trait.html#%E5%9C%A8%E5%A4%96%E9%83%A8%E7%B1%BB%E5%9E%8B%E4%B8%8A%E5%AE%9E%E7%8E%B0%E5%A4%96%E9%83%A8%E7%89%B9%E5%BE%81newtype。

// 🌟🌟
// use std::fmt;

// // 定义一个 newtype `Pretty`


// impl fmt::Display for Pretty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\"{}\"", self.0.clone() + ", world")
//     }
// }

// fn main() {
//     let w = Pretty("hello".to_string());
//     println!("w = {}", w);
// }
use std::fmt;

// 定义一个 newtype `Pretty`
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}