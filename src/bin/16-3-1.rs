// 格式化输出
// 位置参数
// 🌟🌟
// /* 填空 */
// fn main() {
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");// => Alice, this is Bob. Bob, this is Alice
//     assert_eq!(format!("{1}{0}", 1, 2), __);
//     assert_eq!(format!(__, 1, 2), "2112");
//     println!("Success!");
// }

/* 填空 */
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");// => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112");
    println!("Success!");
}