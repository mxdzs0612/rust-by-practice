// 内部表示
// 事实上 String 是一个智能指针，它作为一个结构体存储在栈上，然后指向存储在堆上的字符串底层数据。

// 存储在栈上的智能指针结构体由三部分组成：一个指针只指向堆上的字节数组，已使用的长度以及已分配的容量 capacity (已使用的长度小于等于已分配的容量，当容量不够时，会重新分配内存空间)。

// 🌟🌟 如果 String 的当前容量足够，那么添加字符将不会导致新的内存分配

// // 修改下面的代码以打印如下内容: 
// // 25
// // 25
// // 25
// // 循环中不会发生任何内存分配
// fn main() {
//     let mut s = String::new();

//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }

//     println!("Success!")
// }


// 修改下面的代码以打印如下内容: 
// 25
// 25
// 25
// 循环中不会发生任何内存分配
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}