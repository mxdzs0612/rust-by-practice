// 捕获环境中的值
// 🌟🌟🌟
// fn get_person() -> String {
//     String::from("sunface")
// }

// fn get_format() -> (usize, usize) {
//     (4, 1)
// }


// fn main() {
//     let person = get_person();
//     println!("Hello, {person}!");

//     let (width, precision) = get_format();
//     let scores = [("sunface", 99.12), ("jack", 60.34)];
//     /* 让下面的代码输出:
//     sunface: 99.1
//     jack: 60.3
//     */
//     for (name, score) in scores {
//         println!("{name}: __");
//     }
// }

fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}


fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    /* 让下面的代码输出:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        println!("{name}: {score:.1}");
        // println!("{name}: {score:width$.precision$}");
    }
}

// fn main() {
//     // 指数
//     println!("{:2e}", 1000000000); // => 1e9
//     println!("{:2E}", 1000000000); // => 1E9

//     // 指针地址
//     let v= vec![1, 2, 3];
//     println!("{:p}", v.as_ptr()); // => 0x600002324050

//     // 转义
//     println!("Hello {{}}"); // => Hello {}
// }