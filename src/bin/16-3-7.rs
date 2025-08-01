// 🌟🌟🌟 字符串长度
// fn main() {
//     let s = "Hello, world!";

//     println!("{0:.5}", s); // => Hello

//     assert_eq!(format!("Hello __!", 3, "abcdefg"), "Hello abc!");

//     println!("Success!")
// }

fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");

    println!("Success!")
}