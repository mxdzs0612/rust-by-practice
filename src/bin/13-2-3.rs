// 🌟🌟

// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// // 填空
// // 不要修改其它代码
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

//     __;

//     Ok(s)
// }

// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!")
// }


use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 填空
// 不要修改其它代码
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!")
}