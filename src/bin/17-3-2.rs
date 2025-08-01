// 🌟🌟
// /* 添加类型约束使下面代码正常运行 */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     println!("Success!")
// }

/* 添加类型约束使下面代码正常运行 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a : 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}