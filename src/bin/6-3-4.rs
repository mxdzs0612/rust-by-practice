// 字符串切片
// 🌟
// fn main() {
//     let s = String::from("hello");

//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[__];

//     assert_eq!(slice1, slice2);
// }


fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];
    // let slice2 = &s[0..=1];

    assert_eq!(slice1, slice2);
}