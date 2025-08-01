// 引用和借用 引用
// 🌟

// fn main() {
//    let x = 5;
//    // 填写空白处
//    let p = __;

//    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }

fn main() {
   let x = 5;
   // 填写空白处
   let p = &x;

   println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}