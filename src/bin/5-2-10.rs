// NLL
// 🌟🌟 

// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
    
//     println!("{}",r1);
// }

// 注释掉一行代码让它工作
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1);
    println!("{}",r2);
}