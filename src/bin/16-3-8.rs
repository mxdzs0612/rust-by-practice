// 二进制, 八进制, 十六进制
// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) -> "0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"
// 🌟🌟
// fn main() {
//     assert_eq!(format!("__", 27), "0b11011");
//     assert_eq!(format!("__", 27), "0o33");
//     assert_eq!(format!("__", 27), "0x1b");
//     assert_eq!(format!("__", 27), "0x1B");

//     println!("{:x}!", 27); // 没有前缀的十六进制 => 1b

//     println!("{:#010b}", 27); // 使用 0 来填充二进制，宽度为 10 => 0b00011011

//     println!("Success!")
// }

fn main() {
    assert_eq!(format!("0b{:b}", 27), "0b11011");
    assert_eq!(format!("0o{:o}", 27), "0o33");
    assert_eq!(format!("0x{:x}", 27), "0x1b");
    assert_eq!(format!("0x{:X}", 27), "0x1B");

    println!("{:x}!", 27); // 没有前缀的十六进制 => 1b

    println!("{:#010b}", 27); // 使用 0 来填充二进制，宽度为 10 => 0b00011011

    println!("Success!")
}