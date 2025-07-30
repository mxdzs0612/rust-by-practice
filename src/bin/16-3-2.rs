// 具名参数
// 🌟🌟
// fn main() {
//     println!("{argument}", argument = "test"); // => "test"

//     /* 填空 */
//     assert_eq!(format!("{name}{}", 1, __), "21");
//     assert_eq!(format!(__,a = "a", b = 'b', c = 3 ), "a 3 b");

//     /* 修复错误 */
//     // 具名参数必须放在其它参数后面
//     println!("{abc} {1}", abc = "def", 2);

//     println!("Success!")
// }

fn main() {
    println!("{argument}", argument = "test"); // => "test"

    /* 填空 */
    assert_eq!(format!("{name}{}", 1, name = "2"), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

    /* 修复错误 */
    // 具名参数必须放在其它参数后面
    println!("{abc} {0}", 2, abc = "def");

    println!("Success!")
}
